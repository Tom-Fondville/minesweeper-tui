use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use crate::game::app::{App, AppState};

pub struct TerminalExitingState {}
impl TerminalExitingState {
    pub fn draw(terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| {
            frame.render_widget("press enter if you realy want to quit", frame.area())
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Enter => app.change_current_state(AppState::Exiting),
                KeyCode::Char('q') => app.change_current_state(AppState::Exiting),
                _ => match app.get_last_state() {
                    Some(last_state) => app.change_current_state(last_state.clone()),
                    None => app.change_current_state(AppState::MainMenu),
                },
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
