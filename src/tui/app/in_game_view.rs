use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, widgets::Widget};

use crate::{
    game::{CellAction, Game, difficulty::Difficulty, position::Position},
    tui::{
        app::{App, AppState},
        ui::in_game_ui::InGameUi,
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

#[derive(PartialEq)]
pub enum InGameViewPopup {
    HelpMenu,
    GameStatus,
    QuitConfirmation,
}

pub struct InGameView {
    game: Game,
    cursor_position: CursorPositon,
    displayed_popup: Option<InGameViewPopup>,
}

impl Default for InGameView {
    fn default() -> Self {
        Self {
            game: Game::new(Difficulty::Easy),
            cursor_position: CursorPositon::defualt(),
            displayed_popup: None,
        }
    }
}
impl InGameView {
    pub fn restart(&mut self) {
        self.game.restart();
        self.cursor_position = CursorPositon::defualt();
        self.displayed_popup = None
    }

    pub fn change_difficulty(&mut self, difficulty: Difficulty) {
        self.game.change_difficulty(difficulty);
        self.cursor_position = CursorPositon::defualt();
        self.displayed_popup = None;
    }

    fn reveal_cell(&mut self) {
        self.game.act(CellAction::UnCover(Position::new(
            self.cursor_position.row,
            self.cursor_position.column,
        )));

        if !self.game.board.is_game_running() {
            self.display_popup(InGameViewPopup::GameStatus);
        }
    }

    fn toggle_flag(&mut self) {
        self.game.act(CellAction::Mark(Position::new(
            self.cursor_position.row,
            self.cursor_position.column,
        )));
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

    fn toggle_popup(&mut self, popup: InGameViewPopup) {
        if let Some(displayed_popup) = &self.displayed_popup
            && *displayed_popup == popup
        {
            self.displayed_popup = None
        }

        self.displayed_popup = Some(popup)
    }

    fn display_popup(&mut self, popup: InGameViewPopup) {
        self.displayed_popup = Some(popup)
    }

    fn hide_popup(&mut self) {
        self.displayed_popup = None
    }

    pub fn draw(&self, terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| {
            InGameUi::new(
                &self.game,
                &self.cursor_position,
                self.displayed_popup.as_ref(),
            )
            .render(frame.area(), frame.buffer_mut());
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match &app.in_game_view.displayed_popup {
                Some(dispayed_popup) => match *dispayed_popup {
                    InGameViewPopup::HelpMenu => {
                        Self::handle_key_event_for_help_menu(app, key_event);
                    }
                    InGameViewPopup::GameStatus => {
                        Self::handle_key_event_for_game_status_popup(app, key_event)
                    }
                    InGameViewPopup::QuitConfirmation => {
                        Self::handle_key_event_for_quit_confirmation_popup(app, key_event)
                    }
                },
                None => Self::handle_key_event_default(app, key_event),
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }

    fn handle_key_event_default(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
            KeyCode::Esc => app
                .in_game_view
                .display_popup(InGameViewPopup::QuitConfirmation),

            KeyCode::Char('h') => app.in_game_view.move_cursor(MoveDirection::Left),
            KeyCode::Char('j') => app.in_game_view.move_cursor(MoveDirection::Down),
            KeyCode::Char('k') => app.in_game_view.move_cursor(MoveDirection::Up),
            KeyCode::Char('l') => app.in_game_view.move_cursor(MoveDirection::Right),

            KeyCode::Char('f') => app.in_game_view.toggle_flag(),
            KeyCode::Char('r') => app.in_game_view.restart(),
            KeyCode::Enter => app.in_game_view.reveal_cell(),
            KeyCode::Char(' ') => app.in_game_view.reveal_cell(),
            KeyCode::Char('?') => app.in_game_view.display_popup(InGameViewPopup::HelpMenu),
            _ => {}
        }
    }

    fn handle_key_event_for_game_status_popup(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
            KeyCode::Char('r') => app.in_game_view.restart(),
            KeyCode::Char('h') => app.in_game_view.toggle_popup(InGameViewPopup::GameStatus),
            KeyCode::Esc => app.change_current_state(AppState::MainMenu),
            _ => (),
        }
    }

    fn handle_key_event_for_quit_confirmation_popup(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
            KeyCode::Enter => {
                app.change_current_state(AppState::MainMenu);
            }
            _ => app.in_game_view.hide_popup(),
        }
    }

    fn handle_key_event_for_help_menu(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => app.in_game_view.hide_popup(),
            KeyCode::Char('?') => app.in_game_view.hide_popup(),
            _ => (),
        }
    }
}
