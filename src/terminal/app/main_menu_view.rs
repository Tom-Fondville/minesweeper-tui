use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    widgets::{ListState, StatefulWidget},
};

use crate::{
    game::board::Difficulty,
    terminal::{
        app::{App, AppState},
        ui::{difficulty_ui::DifficultyUi, main_menu_ui::MainMenuUi},
    },
};

pub struct MainMenuView {
    difficulties: [DifficultyUi; 4],
    list_state: ListState,
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

    pub fn draw(&mut self, terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| {
            MainMenuUi::new(&self.difficulties).render(
                frame.area(),
                frame.buffer_mut(),
                &mut self.list_state,
            )
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Char('j') => app.main_menu_state.select_next_difficulty(),
                KeyCode::Char('k') => app.main_menu_state.select_previous_difficulty(),
                KeyCode::Enter => {
                    let difficulty = match app.main_menu_state.get_selected_difficulty() {
                        DifficultyUi::Easy => Difficulty::Easy,
                        DifficultyUi::Medium => Difficulty::Medium,
                        DifficultyUi::Hard => Difficulty::Hard,
                        DifficultyUi::Custom => todo!(),
                    };

                    app.start_new_board(difficulty);
                }
                KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
                _ => (),
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
