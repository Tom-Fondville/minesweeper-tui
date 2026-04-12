use crossterm::event::{self, Event, KeyEvent};
use ratatui::{DefaultTerminal, init, restore};
use std::{
    io::{self},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub mod exiting_view;
pub mod in_game_view;
pub mod main_menu_view;

use crate::tui::app::{
    exiting_view::ExitingView, in_game_view::InGameView, main_menu_view::MainMenuView,
};

#[derive(Clone)]
pub enum AppState {
    MainMenu,
    InGame,
    Exiting,
}

pub enum AppEvent {
    Input(KeyEvent),
    Timer,
    Resize,
}

pub struct App {
    pub current_state: AppState,
    pub last_state: Option<AppState>,
    pub main_menu_state: MainMenuView,
    pub in_game_view: InGameView,
    pub need_exit: bool,
    event_sender: Sender<AppEvent>,
    event_receiver: Receiver<AppEvent>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::channel::<AppEvent>();
        Self {
            current_state: AppState::MainMenu,
            last_state: None,
            main_menu_state: MainMenuView::default(),
            in_game_view: InGameView::default(),
            need_exit: false,
            event_sender,
            event_receiver,
        }
    }

    pub fn start(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;

        let key_input_event_sender = self.event_sender.clone();
        thread::spawn(move || {
            loop {
                let Ok(event) = event::read() else {
                    continue;
                };

                match event {
                    Event::Key(key_event) => key_input_event_sender
                        .send(AppEvent::Input(key_event))
                        .unwrap(),
                    Event::Resize(_, _) => key_input_event_sender.send(AppEvent::Resize).unwrap(),
                    Event::FocusGained => (),
                    Event::FocusLost => (),
                    Event::Mouse(_) => (),
                    Event::Paste(_) => (),
                }
            }
        });

        let timer_event_sender = self.event_sender.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                timer_event_sender.send(AppEvent::Timer).unwrap()
            }
        });
        let mut terminal = init();
        let _ = self.run(&mut terminal);
        restore();

        Ok(())
    }

    pub fn change_current_state(&mut self, state: AppState) {
        self.last_state = Some(self.current_state.clone());
        self.current_state = state;
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            self.handle_tick(terminal);

            if self.need_exit {
                break Ok(());
            }
        }
    }

    fn handle_tick(&mut self, terminal: &mut DefaultTerminal) {
        match self.current_state {
            AppState::MainMenu => self.main_menu_state.draw(terminal),
            AppState::Exiting => ExitingView::draw(terminal),
            AppState::InGame => self.in_game_view.draw(terminal),
        }

        match self.event_receiver.recv().unwrap() {
            AppEvent::Input(key_event) => match self.current_state {
                AppState::MainMenu => MainMenuView::handle_key_event(self, key_event),
                AppState::Exiting => ExitingView::handle_key_event(self, key_event),
                AppState::InGame => InGameView::handle_key_event(self, key_event),
            },
            AppEvent::Timer => (),
            AppEvent::Resize => (),
        }
    }
}
