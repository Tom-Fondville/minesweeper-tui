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

enum TerminalAppState {
    MainMenu,
    InGame,
    Exiting,
}

pub struct TerminalApp<'a> {
    app: App,
    terminal_app_state: TerminalAppState,

    in_game_state: Option<TerminalInGameState<'a>>,
}

impl<'a> TerminalApp<'a> {
    pub fn new() -> Self {
        Self {
            app: App::new(),
            terminal_app_state: TerminalAppState::MainMenu,
            in_game_state: None,
        }
    }

    pub fn start(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        let _ = ratatui::run(|terminal| self.run(terminal));
        Ok(())
    }

    fn change_current_state(&mut self, terminal_state: TerminalAppState) {
        let state = match terminal_state {
            TerminalAppState::MainMenu => AppState::MainMenu,
            TerminalAppState::InGame => AppState::InGame,
            TerminalAppState::Exiting => AppState::Exiting,
        };

        self.app.change_current_state(state);
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
            AppState::Exiting => TerminalExitingState::draw(terminal),
            AppState::InGame => {
                TerminalInGameState::new(self.app.get_current_board()).draw(terminal)
            }
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
                AppState::Exiting => {
                    TerminalExitingState::handle_key_event(&mut self.app, key_event)
                }
                AppState::InGame => match &mut self.in_game_state {
                    Some(state) => state.handle_key_event(&mut self.app, key_event),
                    None => self.change_current_state(TerminalAppState::MainMenu),
                },
            }
        }
    }
}
