use crossterm::event::KeyEvent;
use ratatui::DefaultTerminal;
use std::{
    io::{self},
    sync::mpsc::Receiver,
};

pub mod exiting_view;
pub mod in_game_view;
pub mod main_menu_view;

use crate::{
    game::difficulty::Difficulty,
    tui::app::{exiting_view::ExitingView, in_game_view::InGameView, main_menu_view::MainMenuView},
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
}

pub struct App {
    pub current_state: AppState,
    pub last_state: Option<AppState>,
    pub main_menu_state: MainMenuView,
    pub in_game_view: InGameView,
    pub need_exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            current_state: AppState::MainMenu,
            last_state: None,
            main_menu_state: MainMenuView::default(),
            in_game_view: InGameView::default(),
            need_exit: false,
        }
    }

    pub fn start(&mut self, app_event_receiver: &Receiver<AppEvent>) -> color_eyre::Result<()> {
        color_eyre::install()?;
        let _ = ratatui::run(|terminal| self.run(terminal, app_event_receiver));
        Ok(())
    }

    pub fn change_current_state(&mut self, state: AppState) {
        self.last_state = Some(self.current_state.clone());
        self.current_state = state;
    }

    pub fn start_new_board(&mut self, difficulty: Difficulty) {
        self.in_game_view.change_difficulty(difficulty);
        self.change_current_state(AppState::InGame);
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        app_event_receiver: &Receiver<AppEvent>,
    ) -> io::Result<()> {
        loop {
            self.handle_tick(terminal, app_event_receiver);

            if self.need_exit {
                break Ok(());
            }
        }
    }

    fn handle_tick(
        &mut self,
        terminal: &mut DefaultTerminal,
        app_event_receiver: &Receiver<AppEvent>,
    ) {
        match self.current_state {
            AppState::MainMenu => self.main_menu_state.draw(terminal),
            AppState::Exiting => ExitingView::draw(terminal),
            AppState::InGame => self.in_game_view.draw(terminal),
        }

        match app_event_receiver.recv().unwrap() {
            AppEvent::Input(key_event) => match self.current_state {
                AppState::MainMenu => MainMenuView::handle_key_event(self, key_event),
                AppState::Exiting => ExitingView::handle_key_event(self, key_event),
                AppState::InGame => InGameView::handle_key_event(self, key_event),
            },
            AppEvent::Timer => (),
        }

        // let event = event::read();
        // if event.is_err() {
        //     panic!()
        // }
        //
        // if let Event::Key(key_event) = event.unwrap() {
        //     match self.current_state {
        //         AppState::MainMenu => MainMenuView::handle_key_event(self, key_event),
        //         AppState::Exiting => ExitingView::handle_key_event(self, key_event),
        //         AppState::InGame => InGameView::handle_key_event(self, key_event),
        //     }
        // }
    }
}
