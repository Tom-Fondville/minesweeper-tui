use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io::{self};

use crate::{
    game::app::{App, AppState},
    terminal::terminal_state::{
        terminal_exiting_state::TerminalExitingState,
        terminal_in_game_state::TerminalMainMenuState,
        terminal_main_menu_state::TerminalInGameState,
    },
};

pub struct TerminalApp {}

impl TerminalApp {
    pub fn start() -> color_eyre::Result<()> {
        color_eyre::install()?;
        let _ = ratatui::run(|terminal| Self::run(terminal, &mut App::new()));
        Ok(())
    }

    fn run(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
        loop {
            Self::handle_tick(app, terminal);

            if app.need_to_exit() {
                break Ok(());
            }
        }
    }

    fn handle_tick(app: &mut App, terminal: &mut DefaultTerminal) {
        match app.get_current_state() {
            AppState::MainMenu => TerminalMainMenuState::draw(terminal),
            AppState::Exiting(_) => TerminalExitingState::draw(terminal),
            AppState::InGame(board) => TerminalInGameState::draw(terminal, board),
        }

        let event = event::read();
        if event.is_err() {
            panic!()
        }

        if let Event::Key(key_event) = event.unwrap() {
            match app.get_current_state() {
                AppState::MainMenu => TerminalMainMenuState::handle_key_event(app, key_event),
                AppState::Exiting(_) => TerminalExitingState::handle_key_event(app, key_event),
                AppState::InGame(_) => TerminalInGameState::handle_key_event(app, key_event),
            }
        }
    }
}
