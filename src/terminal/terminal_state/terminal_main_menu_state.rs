use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{
    game::board::Difficulty,
    terminal::app::{App, AppState},
};

#[derive(Clone, Copy)]
pub enum DifficultyUi {
    Easy,
    Medium,
    Hard,
    Custom,
}

impl DifficultyUi {
    pub fn as_str(&self) -> String {
        let str = match self {
            DifficultyUi::Easy => "Easy",
            DifficultyUi::Medium => "Medium",
            DifficultyUi::Hard => "Hard",
            DifficultyUi::Custom => "Custom",
        };
        str.to_string()
    }
}

pub struct TerminalMainMenuState {
    difficulties: [DifficultyUi; 4],
    selected_index: usize,
}

impl Default for TerminalMainMenuState {
    fn default() -> Self {
        Self {
            difficulties: [
                DifficultyUi::Easy,
                DifficultyUi::Medium,
                DifficultyUi::Hard,
                DifficultyUi::Custom,
            ],
            selected_index: 0,
        }
    }
}

impl TerminalMainMenuState {
    fn select_next_difficulty(&mut self) {
        if self.selected_index == 3 {
            return;
        }
        self.selected_index += 1;
    }
    fn select_previous_difficulty(&mut self) {
        if self.selected_index == 0 {
            return;
        }
        self.selected_index -= 1;
    }

    fn get_selected_difficulty(&self) -> &DifficultyUi {
        &self.difficulties[self.selected_index]
    }

    pub fn draw(&self, terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| {
            let chunks = ratatui::layout::Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(frame.area());
            frame.render_widget("Press enter to play. Press q to quit", chunks[0]);

            let mut difficulty_items = Vec::<ListItem>::new();
            for difficulty in self.difficulties {
                difficulty_items.push(ListItem::new(difficulty.as_str()));
            }

            let mut list_state = ListState::default();
            list_state.select(Some(self.selected_index));

            let list = List::new(difficulty_items)
                .block(
                    Block::default()
                        .title("Select Difficulty")
                        .borders(Borders::ALL),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");
            frame.render_stateful_widget(list, chunks[1], &mut list_state);

            let footer = format!("selected: {}", self.get_selected_difficulty().as_str());
            frame.render_widget(footer, chunks[2]);
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
