use crate::game::Game;

#[derive(Debug)]
pub enum AppState {
    Main,
    Exiting,
}

#[derive(Debug)]
pub struct App {
    pub current_screen: AppState,
    pub game: Option<Game>,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        return Self {
            current_screen: AppState::Main,
            game: None,
            exit: false,
        };
    }
}
