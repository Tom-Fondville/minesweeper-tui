use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use crate::{
    game::board::Difficulty,
    terminal::app::{App, AppState},
};

pub struct TerminalMainMenuState {}
impl TerminalMainMenuState {
    pub fn draw(terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| {
            frame.render_widget("Press enter to play. Press q to quit", frame.area())
        });
    }
}

impl TerminalMainMenuState {
    pub fn handle_key_event(terminal_app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Enter => {
                    //TODO: handle difficulty choose
                    // let board = self.store.get_current_board(Difficulty::Easy);

                    terminal_app.start_new_board(Difficulty::Easy);
                }
                KeyCode::Char('q') => terminal_app.change_current_state(AppState::Exiting),
                _ => (),
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
