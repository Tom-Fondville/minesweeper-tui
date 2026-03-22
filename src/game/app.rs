use crate::game::board::{Board, Difficulty};

#[derive(Debug, Clone)]
pub enum AppState {
    MainMenu,
    InGame,
    Exiting,
}

#[derive(Debug)]
pub struct App {
    current_state: AppState,
    last_state: Option<AppState>,
    board: Option<Board>,
    need_exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_state: AppState::MainMenu,
            last_state: None,
            board: None,
            need_exit: false,
        }
    }

    pub fn get_current_state(&self) -> &AppState {
        &self.current_state
    }

    pub fn get_last_state(&self) -> &Option<AppState> {
        &self.last_state
    }

    pub fn get_current_board(&mut self) -> &Board {
        self.board.get_or_insert(Board::new(Difficulty::Easy))
    }

    pub fn need_to_exit(&self) -> bool {
        match self.current_state {
            AppState::Exiting => self.need_exit,
            _ => false,
        }
    }

    pub fn change_current_state(&mut self, state: AppState) {
        self.last_state = Some(self.current_state.clone());
        self.current_state = state;
    }

    pub fn start_new_board(&mut self, difficulty: Difficulty) {
        self.board = Some(Board::new(difficulty));
        self.change_current_state(AppState::InGame);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
