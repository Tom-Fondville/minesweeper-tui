use crate::game::Game;

#[derive(Debug)]
pub enum AppState {
    MainMenu,
    Exiting,
}

#[derive(Debug)]
pub struct App {
    pub current_state: AppState,
    pub game: Option<Game>,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        return Self {
            current_state: AppState::MainMenu,
            game: None,
            exit: false,
        };
    }
}
