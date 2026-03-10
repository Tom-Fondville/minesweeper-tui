use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, style::Stylize, text::Line, widgets::Block};

use crate::game::{
    app::{App, AppState},
    board::{Board, Difficulty},
};

pub struct TerminalMainMenuState {}
impl TerminalMainMenuState {
    pub fn draw(terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| {
            frame.render_widget("Press enter to play. Press q to quit", frame.area())
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Enter => {
                    app.change_current_state(AppState::InGame(Board::new(Difficulty::Easy)))
                }
                KeyCode::Char('q') => app.change_current_state(AppState::Exiting(false)),
                _ => (),
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
