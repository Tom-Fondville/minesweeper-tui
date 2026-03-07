use std::io::{self};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use crate::app::{App, AppState};

pub struct TerminalApp {}

impl TerminalApp {
    pub fn start() -> color_eyre::Result<()> {
        color_eyre::install()?;
        let _ = ratatui::run(|terminal| Self::run(terminal, &mut App::new()));
        Ok(())
    }

    fn run(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
        loop {
            // let terminal_state: TerminalState = match app.current_state {
            //     AppState::MainMenu => TerminalMainMenuState {},
            //     AppState::Exiting => TerminalExitingState {},
            // };
            // Self::handle_tick(app, terminal_state, terminal);

            Self::handle_tick(app, terminal);

            if app.exit {
                break Ok(());
            }
        }
    }

    fn handle_tick(
        app: &mut App,
        // terminal_state: impl TerminalState,
        terminal: &mut DefaultTerminal,
    ) {
        // I can figure how to do generic things yet, so for the moment i will go with match
        // statements and keep my Trait thing
        // terminal_state.draw(terminal, app);
        match app.current_state {
            AppState::MainMenu => TerminalMainMenuState::draw(terminal, app),
            AppState::Exiting => TerminalExitingState::draw(terminal, app),
        }

        let event = event::read();
        if event.is_err() {
            panic!()
        }

        if let Event::Key(key_event) = event.unwrap() {
            match app.current_state {
                AppState::MainMenu => {
                    TerminalMainMenuState::handle_key_event(terminal, app, key_event)
                }
                AppState::Exiting => {
                    TerminalExitingState::handle_key_event(terminal, app, key_event)
                }
            }
        }

        //TODO: return the new state instead of this
        // match app.current_state {
        //     AppState::MainMenu => TerminalMainMenuState {},
        //     AppState::Exiting => TerminalExitingState {},
        // }
        // terminal_state
    }
}

pub trait TerminalState {
    fn draw(terminal: &mut DefaultTerminal, app: &mut App);
    fn handle_key_event(terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent);
    // fn draw(&self, terminal: &mut DefaultTerminal, app: &mut App);
    // fn handle_key_event(&self, terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent);
}

pub struct TerminalMainMenuState {}
impl TerminalState for TerminalMainMenuState {
    fn draw(terminal: &mut DefaultTerminal, app: &mut App) {
        let _ = terminal.draw(|frame| frame.render_widget("hello world", frame.area()));
    }

    fn handle_key_event(terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => {
                if let KeyCode::Char('q') = key_event.code {
                    app.current_state = AppState::Exiting
                }
            }
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}

pub struct TerminalExitingState {}
impl TerminalState for TerminalExitingState {
    fn draw(terminal: &mut DefaultTerminal, app: &mut App) {
        let _ = terminal.draw(|frame| {
            frame.render_widget("press enter if you realy want to quit", frame.area())
        });
    }

    fn handle_key_event(terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Enter => app.exit = true,
                // KeyCode::Enter => panic!(),
                _ => app.current_state = AppState::MainMenu,
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
