use crate::game::board::{Board, Difficulty};

pub mod board;

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
