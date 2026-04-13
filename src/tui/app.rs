use crossterm::event::{self, Event, KeyEvent};
use ratatui::{init, restore};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub mod exiting_view;
pub mod in_game_view;
pub mod main_menu_view;
pub mod popup_view_selector;

use crate::tui::app::{in_game_view::InGameView, main_menu_view::MainMenuView};

#[derive(Clone)]
pub enum AppState {
    MainMenu,
    InGame,
}

pub enum AppEvent {
    Input(KeyEvent),
    Timer,
    Resize,
}

pub struct App {
    pub current_state: AppState,
    pub last_state: Option<AppState>,
    pub main_menu_view: MainMenuView,
    pub in_game_view: InGameView,
    pub need_exit: bool,
    event_sender: Sender<AppEvent>,
    event_receiver: Receiver<AppEvent>,
}

impl Default for App {
    fn default() -> Self {
        let (event_sender, event_receiver) = mpsc::channel::<AppEvent>();
        Self {
            current_state: AppState::MainMenu,
            last_state: None,
            main_menu_view: MainMenuView::default(),
            in_game_view: InGameView::default(),
            need_exit: false,
            event_sender,
            event_receiver,
        }
    }
}

impl App {
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
        loop {
            match self.current_state {
                AppState::MainMenu => self.main_menu_view.draw(&mut terminal),
                AppState::InGame => self.in_game_view.draw(&mut terminal),
            }

            match self.event_receiver.recv().unwrap() {
                AppEvent::Input(key_event) => match self.current_state {
                    AppState::MainMenu => MainMenuView::handle_key_event(self, key_event),
                    AppState::InGame => InGameView::handle_key_event(self, key_event),
                },
                AppEvent::Timer => (),
                AppEvent::Resize => (),
            }

            if self.need_exit {
                break;
            }
        }
        restore();

        Ok(())
    }

    pub fn change_current_state(&mut self, state: AppState) {
        self.last_state = Some(self.current_state.clone());
        self.current_state = state;
    }
}
