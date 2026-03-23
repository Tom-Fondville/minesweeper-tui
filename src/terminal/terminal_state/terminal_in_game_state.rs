use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Span},
};

use crate::{
    game::board::Board,
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
}

pub struct TerminalInGameState {
    board: Board,
    cursor_position: CursorPositon,
}

impl TerminalInGameState {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            cursor_position: CursorPositon::new(0, 0),
        }
    }

    fn move_cursor(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::Up => {
                let new_row = u16::saturating_sub(self.cursor_position.row, 1);
                self.cursor_position = CursorPositon::new(new_row, self.cursor_position.column);
            }
            MoveDirection::Down => {
                let new_row = u16::saturating_add(self.cursor_position.row, 1);
                if new_row > self.board.get_rows_count() - 1 {
                    return;
                }
                self.cursor_position = CursorPositon::new(new_row, self.cursor_position.column);
            }
            MoveDirection::Right => {
                let new_column = u16::saturating_add(self.cursor_position.column, 1);
                if new_column > self.board.get_columns_count() + 1 {
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
                " {} ",
                self.board.get_difficulty().as_string().bold()
            ))
            .centered();
            frame.render_widget(title, header);

            frame.render_widget(BoardUi::new(&self.board, &self.cursor_position), body);

            let footer_text = "minesweeper-tui ".italic();
            let coords_text = Span::from(format!(
                "{}, {}",
                self.cursor_position.row, self.cursor_position.column
            ));
            frame.render_widget(Line::from(vec![footer_text, coords_text]), footer);
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
                KeyCode::Char('h') => app.in_game_state.move_cursor(MoveDirection::Left),
                KeyCode::Char('j') => app.in_game_state.move_cursor(MoveDirection::Down),
                KeyCode::Char('k') => app.in_game_state.move_cursor(MoveDirection::Up),
                KeyCode::Char('l') => app.in_game_state.move_cursor(MoveDirection::Right),
                _ => {}
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
