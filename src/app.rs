use crate::game::board::Board;

#[derive(Debug, Clone)]
pub enum AppState {
    MainMenu,
    InGame(Board),
    Exiting(bool),
}

#[derive(Debug)]
pub struct App {
    current_state: AppState,
    last_state: Option<AppState>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_state: AppState::MainMenu,
            last_state: None,
        }
    }

    pub fn get_current_state(&self) -> &AppState {
        &self.current_state
    }

    pub fn get_last_state(&self) -> &Option<AppState> {
        &self.last_state
    }

    pub fn need_to_exit(&self) -> bool {
        match self.current_state {
            AppState::Exiting(exit) => exit,
            _ => false,
        }
    }
    pub fn change_current_state(&mut self, state: AppState) {
        self.last_state = Some(self.current_state.clone());
        self.current_state = state;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
