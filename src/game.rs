use std::time::{Duration, Instant};

use crate::game::{board::Board, difficulty::Difficulty};

pub mod board;
pub mod cell;
pub mod difficulty;
pub mod position;
pub mod status;

pub struct Game {
    pub board: Board,
    pub start_time: Instant,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            board: Board::new(difficulty),
            start_time: Instant::now(),
        }
    }

    pub fn restart(&mut self) {
        self.board = Board::new(*self.board.get_difficulty())
    }

    pub fn elapsed_time_since_start(&self) -> Duration {
        self.start_time.elapsed()
    }
}
