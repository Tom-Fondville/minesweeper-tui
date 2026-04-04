use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use crate::tui::app::{App, AppState};

pub struct ExitingView {}
impl ExitingView {
    pub fn draw(terminal: &mut DefaultTerminal) {
        let _ = terminal
            .draw(|frame| frame.render_widget("press 'q' if you realy want to quit", frame.area()));
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Char('q') => app.need_exit = true,
                _ => match &app.last_state {
                    Some(last_state) => app.change_current_state(last_state.clone()),
                    None => app.change_current_state(AppState::MainMenu),
                },
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
