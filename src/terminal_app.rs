use std::{io, process::exit};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use crate::app::{App, AppState};

pub struct TerminalApp {}

impl TerminalApp {
    pub fn start() -> color_eyre::Result<()> {
        color_eyre::install()?;
        ratatui::run(|terminal| Self::run(terminal, &mut App::new()));
        Ok(())
    }

    fn run(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
        loop {
            Self::draw(terminal, app);
            match event::read()? {
                Event::Key(key_event) => Self::handle_key_event(terminal, app, key_event),
                // _ => app.exit = true,
                _ => (),
            }

            if app.exit {
                break Ok(());
            }
        }
    }

    //for the moment this code is a bunch of crap, i think the idea could be to have an impl of
    //draw and handle key for each AppState (maybe why trait of think like that, need to read more
    //about it)
    fn draw(terminal: &mut DefaultTerminal, app: &mut App) {
        match app.current_screen {
            AppState::Main => {
                terminal.draw(|frame| frame.render_widget("hello world", frame.area()));
                ()
            }
            AppState::Exiting => app.exit = true,
        }
    }
    fn handle_key_event(terminal: &mut DefaultTerminal, app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => match key_event.code {
                KeyCode::Char('q') => app.exit = true,
                // KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                _ => (),
            },
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => match key_event.code {
                // KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                // KeyCode::Char('q') => app.exit = true,
                _ => (),
            },
        }
    }
}
