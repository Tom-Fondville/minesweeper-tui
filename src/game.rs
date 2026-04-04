use crate::game::{board::Board, difficulty::Difficulty};

pub mod board;
pub mod cell;
pub mod difficulty;
pub mod position;
pub mod status;

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            board: Board::new(difficulty),
        }
    }

    pub fn restart(&mut self) {
        self.board = Board::new(*self.board.get_difficulty())
    }
}
