use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io::{self};

use crate::{
    game::app::{App, AppState},
    terminal::terminal_state::{
        terminal_exiting_state::TerminalExitingState, terminal_in_game_state::TerminalInGameState,
        terminal_main_menu_state::TerminalMainMenuState,
    },
};

pub struct TerminalApp {
    app: App,
}

impl TerminalApp {
    pub fn new() -> Self {
        Self { app: App::new() }
    }

    pub fn start(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        let _ = ratatui::run(|terminal| self.run(terminal));
        Ok(())
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            self.handle_tick(terminal);

            if self.app.need_to_exit() {
                break Ok(());
            }
        }
    }

    fn handle_tick(&mut self, terminal: &mut DefaultTerminal) {
        match self.app.get_current_state() {
            AppState::MainMenu => TerminalMainMenuState::draw(terminal),
            AppState::Exiting(_) => TerminalExitingState::draw(terminal),
            AppState::InGame(board) => TerminalInGameState::new(board).draw(terminal),
        }

        let event = event::read();
        if event.is_err() {
            panic!()
        }

        if let Event::Key(key_event) = event.unwrap() {
            match self.app.get_current_state() {
                AppState::MainMenu => {
                    TerminalMainMenuState::handle_key_event(&mut self.app, key_event)
                }
                AppState::Exiting(_) => {
                    TerminalExitingState::handle_key_event(&mut self.app, key_event)
                }
                AppState::InGame(_) => {
                    TerminalInGameState::handle_key_event(&mut self.app, key_event)
                }
            }
        }
    }
}
