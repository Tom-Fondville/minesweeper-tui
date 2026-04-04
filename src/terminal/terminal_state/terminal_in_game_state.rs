use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Span},
};

use crate::{
    game::{
        Game,
        board::{Board, Difficulty, Position},
    },
    terminal::{
        app::{App, AppState},
        ui::board_ui::BoardUi,
    },
};

enum MoveDirection {
    Up,
    Down,
    Right,
    Left,
}

pub struct CursorPositon {
    pub row: u16,
    pub column: u16,
}
impl CursorPositon {
    pub fn new(row: u16, column: u16) -> Self {
        Self { row, column }
    }

    pub fn defualt() -> Self {
        Self { row: 0, column: 0 }
    }
}

pub struct TerminalInGameState {
    game: Game,
    cursor_position: CursorPositon,
}

impl Default for TerminalInGameState {
    fn default() -> Self {
        Self {
            game: Game::new(Difficulty::Easy),
            cursor_position: CursorPositon::defualt(),
        }
    }
}
impl TerminalInGameState {
    pub fn restart(&mut self) {
        self.game.restart();
        self.cursor_position = CursorPositon::defualt()
    }

    pub fn change_difficulty(&mut self, difficulty: Difficulty) {
        self.game.board = Board::new(difficulty);
        self.cursor_position = CursorPositon::defualt();
    }

    fn reveal_cell(&mut self) {
        self.game.board.reveal_cell(&Position::new(
            self.cursor_position.row,
            self.cursor_position.column,
        ));
    }

    fn toggle_flag(&mut self) {
        self.game.board.toggle_flag(&Position::new(
            self.cursor_position.row,
            self.cursor_position.column,
        ));
    }

    fn move_cursor(&mut self, direction: MoveDirection) {
        if !self.game.board.is_game_running() {
            return;
        }

        match direction {
            MoveDirection::Up => {
                let new_row = u16::saturating_sub(self.cursor_position.row, 1);
                self.cursor_position = CursorPositon::new(new_row, self.cursor_position.column);
            }
            MoveDirection::Down => {
                let new_row = u16::saturating_add(self.cursor_position.row, 1);
                if new_row > self.game.board.get_rows_count() - 1 {
                    return;
                }
                self.cursor_position = CursorPositon::new(new_row, self.cursor_position.column);
            }
            MoveDirection::Right => {
                let new_column = u16::saturating_add(self.cursor_position.column, 1);
                if new_column > self.game.board.get_columns_count() - 1 {
                    return;
                }
                self.cursor_position = CursorPositon::new(self.cursor_position.row, new_column);
            }
            MoveDirection::Left => {
                let new_column = u16::saturating_sub(self.cursor_position.column, 1);
                self.cursor_position = CursorPositon::new(self.cursor_position.row, new_column);
            }
        }
    }

    pub fn draw(&self, terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(frame.area());
            let header = chunks[0];
            let body = chunks[1];
            let footer = chunks[2];

            let title = Line::from(format!(
                " {} - {}",
                self.game.board.get_difficulty().as_string().bold(),
                self.game.board.get_status().as_string(),
            ))
            .centered();
            frame.render_widget(title, header);

            frame.render_widget(BoardUi::new(&self.game.board, &self.cursor_position), body);

            let footer_text = "minesweeper-tui ".italic();
            let coords_text = Span::from(format!(
                "{}, {} - bomb number {} flags {}, remaining cells {}",
                self.cursor_position.row,
                self.cursor_position.column,
                self.game.board.get_bomb_number(),
                self.game.board.get_flags_count(),
                self.game.board.remaining_cells_count
            ));
            frame.render_widget(Line::from(vec![footer_text, coords_text]), footer);
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
                //TODO here we should display a popup for confirmation before leaving the game
                KeyCode::Esc => app.change_current_state(AppState::MainMenu),

                KeyCode::Char('h') => app.in_game_state.move_cursor(MoveDirection::Left),
                KeyCode::Char('j') => app.in_game_state.move_cursor(MoveDirection::Down),
                KeyCode::Char('k') => app.in_game_state.move_cursor(MoveDirection::Up),
                KeyCode::Char('l') => app.in_game_state.move_cursor(MoveDirection::Right),

                KeyCode::Char('f') => app.in_game_state.toggle_flag(),
                KeyCode::Char('r') => app.in_game_state.restart(),
                KeyCode::Enter => app.in_game_state.reveal_cell(),
                KeyCode::Char(' ') => app.in_game_state.reveal_cell(),
                _ => {}
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
