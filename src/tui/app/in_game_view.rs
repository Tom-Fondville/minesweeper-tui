use std::time::Instant;

use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, widgets::Widget};

use crate::{
    game::{CellAction, Game, difficulty::Difficulty, position::Position},
    tui::{
        app::{App, AppState, popup_view_selector::PopupViewSelector},
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
    ExitGameConfirmation,
    RestartGameConfirmation,
    ExitAppConfirmation,
}

pub struct InGameView {
    game: Game,
    cursor_position: CursorPositon,
    popup_selector: PopupViewSelector<InGameViewPopup>,
}

impl Default for InGameView {
    fn default() -> Self {
        Self {
            game: Game::new(Difficulty::Easy),
            cursor_position: CursorPositon::defualt(),
            popup_selector: PopupViewSelector::default(),
        }
    }
}
impl InGameView {
    pub fn restart(&mut self) {
        self.game.restart();
        self.cursor_position = CursorPositon::defualt();
        self.popup_selector.hide_popup();
    }

    pub fn change_difficulty(&mut self, difficulty: Difficulty) {
        self.game.change_difficulty(difficulty);
        self.cursor_position = CursorPositon::defualt();
        self.popup_selector.hide_popup();
    }

    fn reveal_cell(&mut self) {
        self.game.act(CellAction::UnCover(Position::new(
            self.cursor_position.row,
            self.cursor_position.column,
        )));

        if !self.game.board.is_game_running() {
            self.popup_selector
                .display_popup(InGameViewPopup::GameStatus);
        }
    }

    fn toggle_flag(&mut self) {
        self.game.act(CellAction::Mark(Position::new(
            self.cursor_position.row,
            self.cursor_position.column,
        )));
    }

    fn move_cursor_to_next_unrevealed_cell(&mut self, direction: MoveDirection) {
        if !self.game.board.is_game_running() {
            return;
        }

        match direction {
            MoveDirection::Up => {
                if self.cursor_position.row == 0 {
                    return;
                }

                let next_unrevealed_cell = self
                    .game
                    .board
                    .get_grid()
                    .iter()
                    .enumerate()
                    .take((self.cursor_position.row) as usize)
                    .rev()
                    .find(|(_, row)| {
                        let cell = row.get(self.cursor_position.column as usize);
                        let Some(cell) = cell else {
                            return false;
                        };

                        !cell.is_revealed()
                    });

                let Some(next_unrevealed_cell) = next_unrevealed_cell else {
                    return;
                };

                self.cursor_position =
                    CursorPositon::new(next_unrevealed_cell.0 as u16, self.cursor_position.column)
            }
            MoveDirection::Down => {
                if self.cursor_position.row == self.game.board.get_rows_count() {
                    return;
                }

                let next_unrevealed_cell = self
                    .game
                    .board
                    .get_grid()
                    .iter()
                    .enumerate()
                    .skip((self.cursor_position.row + 1) as usize)
                    .find(|(_, row)| {
                        let cell = row.get(self.cursor_position.column as usize);
                        let Some(cell) = cell else {
                            return false;
                        };

                        !cell.is_revealed()
                    });

                let Some(next_unrevealed_cell) = next_unrevealed_cell else {
                    return;
                };

                self.cursor_position =
                    CursorPositon::new(next_unrevealed_cell.0 as u16, self.cursor_position.column)
            }
            MoveDirection::Right => {
                if self.cursor_position.column == self.game.board.get_columns_count() {
                    return;
                }

                let current_row = self
                    .game
                    .board
                    .get_grid()
                    .get(self.cursor_position.row as usize);

                let Some(current_row) = current_row else {
                    return;
                };

                let next_unrevealed_cell = current_row
                    .iter()
                    .enumerate()
                    .skip((self.cursor_position.column + 1) as usize)
                    .find(|(_, cell)| !cell.is_revealed());

                let Some(next_unrevealed_cell) = next_unrevealed_cell else {
                    return;
                };

                self.cursor_position =
                    CursorPositon::new(self.cursor_position.row, next_unrevealed_cell.0 as u16)
            }
            MoveDirection::Left => {
                if self.cursor_position.column == 0 {
                    return;
                }

                let current_row = self
                    .game
                    .board
                    .get_grid()
                    .get(self.cursor_position.row as usize);

                let Some(current_row) = current_row else {
                    return;
                };

                let previous_unrevealed_cell = current_row
                    .iter()
                    .enumerate()
                    .take((self.cursor_position.column) as usize)
                    .rev()
                    .find(|(_, cell)| !cell.is_revealed());

                let Some(next_unrevealed_cell) = previous_unrevealed_cell else {
                    return;
                };

                self.cursor_position =
                    CursorPositon::new(self.cursor_position.row, next_unrevealed_cell.0 as u16)
            }
        }
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
            InGameUi::new(
                &self.game,
                &self.cursor_position,
                self.popup_selector.get_selected(),
            )
            .render(frame.area(), frame.buffer_mut());
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match &app.in_game_view.popup_selector.get_selected() {
                Some(dispayed_popup) => match *dispayed_popup {
                    InGameViewPopup::HelpMenu => {
                        Self::handle_key_event_for_help_menu(app, key_event);
                    }
                    InGameViewPopup::GameStatus => {
                        Self::handle_key_event_for_game_status_popup(app, key_event)
                    }
                    InGameViewPopup::ExitGameConfirmation => {
                        Self::handle_key_event_for_exit_game_confirmation_popup(app, key_event)
                    }
                    InGameViewPopup::ExitAppConfirmation => {
                        Self::handle_key_event_for_exit_app_confirmation(app, key_event)
                    }
                    InGameViewPopup::RestartGameConfirmation => {
                        Self::handle_key_for_restart_game_confirmation_popup(app, key_event)
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
            KeyCode::Char('q') => app
                .in_game_view
                .popup_selector
                .toggle_popup(InGameViewPopup::ExitAppConfirmation),
            KeyCode::Esc => {
                app.in_game_view.game.start_pause_time = Some(Instant::now());
                app.in_game_view
                    .popup_selector
                    .display_popup(InGameViewPopup::ExitGameConfirmation)
            }

            KeyCode::Char('h') => app.in_game_view.move_cursor(MoveDirection::Left),
            KeyCode::Char('j') => app.in_game_view.move_cursor(MoveDirection::Down),
            KeyCode::Char('k') => app.in_game_view.move_cursor(MoveDirection::Up),
            KeyCode::Char('l') => app.in_game_view.move_cursor(MoveDirection::Right),

            KeyCode::Left => app.in_game_view.move_cursor(MoveDirection::Left),
            KeyCode::Down => app.in_game_view.move_cursor(MoveDirection::Down),
            KeyCode::Up => app.in_game_view.move_cursor(MoveDirection::Up),
            KeyCode::Right => app.in_game_view.move_cursor(MoveDirection::Right),

            KeyCode::Char('e') => app
                .in_game_view
                .move_cursor_to_next_unrevealed_cell(MoveDirection::Right),
            KeyCode::Char('b') => app
                .in_game_view
                .move_cursor_to_next_unrevealed_cell(MoveDirection::Left),
            KeyCode::Char('{') => app
                .in_game_view
                .move_cursor_to_next_unrevealed_cell(MoveDirection::Up),
            KeyCode::Char('}') => app
                .in_game_view
                .move_cursor_to_next_unrevealed_cell(MoveDirection::Down),

            KeyCode::Char('f') => app.in_game_view.toggle_flag(),
            KeyCode::Char('r') => app
                .in_game_view
                .popup_selector
                .display_popup(InGameViewPopup::RestartGameConfirmation),
            KeyCode::Enter => app.in_game_view.reveal_cell(),
            KeyCode::Char(' ') => app.in_game_view.reveal_cell(),
            KeyCode::Char('?') => app
                .in_game_view
                .popup_selector
                .display_popup(InGameViewPopup::HelpMenu),
            _ => {}
        }
    }

    fn handle_key_event_for_game_status_popup(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => app
                .in_game_view
                .popup_selector
                .toggle_popup(InGameViewPopup::ExitAppConfirmation),
            KeyCode::Char('r') => app.in_game_view.restart(),
            KeyCode::Char('h') => app
                .in_game_view
                .popup_selector
                .toggle_popup(InGameViewPopup::GameStatus),
            KeyCode::Esc => app.change_current_state(AppState::MainMenu),
            _ => (),
        }
    }

    fn handle_key_event_for_exit_game_confirmation_popup(app: &mut App, key_event: KeyEvent) {
        app.in_game_view.game.add_pause_duration();
        match key_event.code {
            KeyCode::Char('q') => app
                .in_game_view
                .popup_selector
                .toggle_popup(InGameViewPopup::ExitAppConfirmation),
            KeyCode::Enter => {
                app.change_current_state(AppState::MainMenu);
            }
            _ => app.in_game_view.popup_selector.hide_popup(),
        }
    }

    fn handle_key_event_for_help_menu(app: &mut App, _: KeyEvent) {
        app.in_game_view.popup_selector.hide_popup()
    }

    fn handle_key_event_for_exit_app_confirmation(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => app.need_exit = true,
            _ => app.in_game_view.popup_selector.hide_popup(),
        }
    }

    fn handle_key_for_restart_game_confirmation_popup(app: &mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('r') => {
                app.in_game_view.game.restart();
                app.in_game_view.popup_selector.hide_popup();
            }
            _ => app.in_game_view.popup_selector.hide_popup(),
        }
    }
}
