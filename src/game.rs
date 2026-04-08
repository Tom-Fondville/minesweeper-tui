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
    pub end_time: Option<Instant>,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            board: Board::new(difficulty),
            start_time: Instant::now(),
            end_time: None,
        }
    }

    pub fn restart(&mut self) {
        self.board = Board::new(*self.board.get_difficulty());
        self.start_time = Instant::now();
    }

    pub fn change_difficulty(&mut self, difficulty: Difficulty) {
        self.board = Board::new(difficulty);
        self.start_time = Instant::now();
    }

    pub fn get_game_duration(&self) -> Duration {
        match self.end_time {
            Some(end_time) => end_time
                .checked_duration_since(self.start_time)
                .unwrap_or(self.start_time.elapsed()),
            None => self.start_time.elapsed(),
        }
    }
}
