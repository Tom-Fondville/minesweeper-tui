use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    widgets::{ListState, StatefulWidget},
};

use crate::{
    game::difficulty::Difficulty,
    tui::{
        app::{App, AppState},
        ui::{difficulty_ui::DifficultyUi, main_menu_ui::MainMenuUi},
    },
};

pub struct MainMenuView {
    difficulties: [DifficultyUi; 4],
    list_state: ListState,
    custom_difficulty_inputs: CustomDifficultyInputs,
    is_help_menu_displayed: bool,
}

impl Default for MainMenuView {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            difficulties: [
                DifficultyUi::Easy,
                DifficultyUi::Medium,
                DifficultyUi::Hard,
                DifficultyUi::Custom,
            ],
            list_state,
            custom_difficulty_inputs: CustomDifficultyInputs::default(),
            is_help_menu_displayed: false,
        }
    }
}

impl MainMenuView {
    fn select_next_difficulty(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous_difficulty(&mut self) {
        self.list_state.select_previous();
    }

    fn get_selected_difficulty(&self) -> &DifficultyUi {
        &self.difficulties[self.list_state.selected().unwrap_or(0)]
    }

    fn is_custom_difficulty_selected(&self) -> bool {
        self.custom_difficulty_inputs.selected_input != SelectedCustomDifficultyInput::None
    }

    fn toggle_help_menu(&mut self) {
        self.is_help_menu_displayed = !self.is_help_menu_displayed
    }

    fn start_new_board(app: &mut App, difficulty: Difficulty) {
        app.in_game_view.change_difficulty(difficulty);
        app.change_current_state(AppState::InGame);
    }

    pub fn exit_help_menu(&mut self) {
        self.is_help_menu_displayed = false;
    }

    pub fn draw(&mut self, terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| {
            MainMenuUi::new(
                &self.difficulties,
                &self.custom_difficulty_inputs,
                &self.custom_difficulty_inputs.selected_input,
                &self.is_help_menu_displayed,
            )
            .render(frame.area(), frame.buffer_mut(), &mut self.list_state)
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        if app.main_menu_state.is_custom_difficulty_selected() {
            Self::handle_key_event_when_custom_difficulty_selected(app, key_event);
            return;
        }

        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Char('j') => app.main_menu_state.select_next_difficulty(),
                KeyCode::Char('k') => app.main_menu_state.select_previous_difficulty(),
                KeyCode::Esc => app.main_menu_state.exit_help_menu(),
                KeyCode::Tab => {
                    let difficulty_ui = app.main_menu_state.get_selected_difficulty();
                    if *difficulty_ui != DifficultyUi::Custom {
                        return;
                    }

                    app.main_menu_state.custom_difficulty_inputs.focus();
                }
                KeyCode::Enter => {
                    let difficulty_ui = app.main_menu_state.get_selected_difficulty();
                    if *difficulty_ui == DifficultyUi::Custom {
                        app.main_menu_state.custom_difficulty_inputs.focus();
                        return;
                    }

                    let difficulty = match *difficulty_ui {
                        DifficultyUi::Easy => Difficulty::Easy,
                        DifficultyUi::Medium => Difficulty::Medium,
                        DifficultyUi::Hard => Difficulty::Hard,
                        DifficultyUi::Custom => unreachable!(),
                    };

                    Self::start_new_board(app, difficulty);
                }
                KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
                KeyCode::Char('?') => app.main_menu_state.toggle_help_menu(),
                _ => (),
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }

    fn handle_key_event_when_custom_difficulty_selected(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Enter => {
                    let Some(difficulty) = app
                        .main_menu_state
                        .custom_difficulty_inputs
                        .create_custom_difficulty()
                    else {
                        return;
                    };

                    Self::start_new_board(app, difficulty);
                }
                KeyCode::Tab => app.main_menu_state.custom_difficulty_inputs.focus_next(),
                KeyCode::BackTab => app
                    .main_menu_state
                    .custom_difficulty_inputs
                    .focus_previous(),
                KeyCode::Backspace => app.main_menu_state.custom_difficulty_inputs.delete(),
                KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
                KeyCode::Char('?') => app.main_menu_state.toggle_help_menu(),
                KeyCode::Char(char) => app.main_menu_state.custom_difficulty_inputs.write(char),
                KeyCode::Esc => app.main_menu_state.custom_difficulty_inputs.un_focus(),
                _ => (),
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}

#[derive(Default, PartialEq)]
pub enum SelectedCustomDifficultyInput {
    #[default]
    None,
    Rows,
    Colomns,
    Bombs,
}

#[derive(Default)]
pub struct CustomDifficultyInputs {
    pub rows: String,
    pub colomns: String,
    pub bombs: String,
    pub selected_input: SelectedCustomDifficultyInput,
}

impl CustomDifficultyInputs {
    pub fn focus(&mut self) {
        self.focus_next();
    }

    pub fn un_focus(&mut self) {
        self.selected_input = SelectedCustomDifficultyInput::None
    }

    pub fn focus_next(&mut self) {
        match self.selected_input {
            SelectedCustomDifficultyInput::None => {
                self.selected_input = SelectedCustomDifficultyInput::Rows
            }
            SelectedCustomDifficultyInput::Rows => {
                self.selected_input = SelectedCustomDifficultyInput::Colomns
            }
            SelectedCustomDifficultyInput::Colomns => {
                self.selected_input = SelectedCustomDifficultyInput::Bombs
            }
            SelectedCustomDifficultyInput::Bombs => {
                self.selected_input = SelectedCustomDifficultyInput::Rows
            }
        }
    }

    pub fn focus_previous(&mut self) {
        match self.selected_input {
            SelectedCustomDifficultyInput::None => {
                self.selected_input = SelectedCustomDifficultyInput::Bombs
            }
            SelectedCustomDifficultyInput::Rows => {
                self.selected_input = SelectedCustomDifficultyInput::Bombs
            }
            SelectedCustomDifficultyInput::Colomns => {
                self.selected_input = SelectedCustomDifficultyInput::Rows
            }
            SelectedCustomDifficultyInput::Bombs => {
                self.selected_input = SelectedCustomDifficultyInput::Colomns
            }
        }
    }

    pub fn write(&mut self, char: char) {
        let Some(_) = char.to_digit(10) else {
            return;
        };

        match self.selected_input {
            SelectedCustomDifficultyInput::None => (),
            SelectedCustomDifficultyInput::Rows => self.rows.push(char),
            SelectedCustomDifficultyInput::Colomns => self.colomns.push(char),
            SelectedCustomDifficultyInput::Bombs => self.bombs.push(char),
        }
    }

    pub fn delete(&mut self) {
        match self.selected_input {
            SelectedCustomDifficultyInput::None => (),
            SelectedCustomDifficultyInput::Rows => {
                self.rows.pop();
            }
            SelectedCustomDifficultyInput::Colomns => {
                self.colomns.pop();
            }
            SelectedCustomDifficultyInput::Bombs => {
                self.bombs.pop();
            }
        }
    }

    fn create_custom_difficulty(&self) -> Option<Difficulty> {
        if self.rows.is_empty() || self.colomns.is_empty() {
            return None;
        }

        Some(Difficulty::Custom {
            rows_number: self.rows.parse::<u16>().ok()?,
            column_number: self.colomns.parse::<u16>().ok()?,
            bomb_number: self.bombs.parse::<u16>().ok()?,
        })
    }
}
