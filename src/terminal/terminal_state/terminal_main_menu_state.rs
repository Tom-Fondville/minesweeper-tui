use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, style::Stylize, text::Line, widgets::Block};

use crate::game::{
    app::{App, AppState},
    board::Board,
};

pub struct TerminalInGameState {}
impl TerminalInGameState {
    pub fn draw(terminal: &mut DefaultTerminal, board: &Board) {
        let title = Line::from(format!(" {} ", board.get_difficulty().as_string().bold()));
        let block = Block::bordered().title(title.centered());

        //TODO: display the grid here

        let _ = terminal.draw(|frame| frame.render_widget(block, frame.area()));
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => {
                if let KeyCode::Char('q') = key_event.code {
                    app.change_current_state(AppState::Exiting(false));
                }
            }
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
