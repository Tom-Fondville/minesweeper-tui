use std::io;

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
            let terminal_state: TerminalState = match app.current_state {
                AppState::MainMenu => TerminalMainMenuState {},
                AppState::Exiting => TerminalExitingState {},
            };
            Self::handle_tick(app, terminal_state, terminal);
        }
    }

    fn handle_tick(
        app: &mut App,
        terminal_state: impl TerminalState,
        terminal: &mut DefaultTerminal,
    ) -> impl TerminalState {
        terminal_state.draw(terminal, app);
        let event = event::read();
        if event.is_err() {
            panic!()
        }
        match event.unwrap() {
            Event::Key(key_event) => terminal_state.handle_key_event(terminal, app, key_event),
            _ => (),
        }

        //TODO: return the new state instead of this
        // match app.current_state {
        //     AppState::MainMenu => TerminalMainMenuState {},
        //     AppState::Exiting => TerminalExitingState {},
        // }
        terminal_state
    }
}

pub trait TerminalState {
    fn draw(&self, terminal: &mut DefaultTerminal, app: &mut App);
    fn handle_key_event(&self, terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent);
}

pub struct TerminalMainMenuState {}
impl TerminalState for TerminalMainMenuState {
    fn draw(&self, terminal: &mut DefaultTerminal, app: &mut App) {
        terminal.draw(|frame| frame.render_widget("hello world", frame.area()));
    }

    fn handle_key_event(&self, terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Char('q') => app.current_state = AppState::Exiting,
                _ => (),
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}

pub struct TerminalExitingState {}
impl TerminalState for TerminalExitingState {
    fn draw(&self, terminal: &mut DefaultTerminal, app: &mut App) {
        let _ = terminal.draw(|frame| {
            frame.render_widget("press enter if you realy want to quit", frame.area())
        });
    }

    fn handle_key_event(&self, terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Enter => app.exit = true,
                _ => app.current_state = AppState::MainMenu,
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
