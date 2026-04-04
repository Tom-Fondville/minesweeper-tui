use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io::{self};

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

pub struct App {
    pub current_state: AppState,
    pub last_state: Option<AppState>,
    pub main_menu_state: MainMenuView,
    pub in_game_state: InGameView,
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
            in_game_state: InGameView::default(),
            need_exit: false,
        }
    }

    pub fn start(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        let _ = ratatui::run(|terminal| self.run(terminal));
        Ok(())
    }

    pub fn change_current_state(&mut self, state: AppState) {
        self.last_state = Some(self.current_state.clone());
        self.current_state = state;
    }

    pub fn start_new_board(&mut self, difficulty: Difficulty) {
        self.in_game_state.change_difficulty(difficulty);
        self.change_current_state(AppState::InGame);
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
            AppState::InGame => self.in_game_state.draw(terminal),
        }

        let event = event::read();
        if event.is_err() {
            panic!()
        }

        if let Event::Key(key_event) = event.unwrap() {
            match self.current_state {
                AppState::MainMenu => MainMenuView::handle_key_event(self, key_event),
                AppState::Exiting => ExitingView::handle_key_event(self, key_event),
                AppState::InGame => InGameView::handle_key_event(self, key_event),
            }
        }
    }
}
