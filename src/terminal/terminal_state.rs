use crossterm::event::KeyEvent;
use ratatui::DefaultTerminal;

use crate::game::app::App;

pub mod terminal_exiting_state;
pub mod terminal_in_game_state;
pub mod terminal_main_menu_state;

pub struct Totot {}
pub trait TerminalState {
    fn draw(terminal: &mut DefaultTerminal, app: &mut App);
    fn handle_key_event(terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent);
}
