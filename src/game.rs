use std::time::{Duration, Instant};

use crate::game::{board::Board, difficulty::Difficulty, position::Position};

pub mod board;
pub mod cell;
pub mod difficulty;
pub mod position;
pub mod status;

pub struct Game {
    pub board: Board,
    pub start_time: Instant,
    pub total_pause_duration: Duration,
    pub start_pause_time: Option<Instant>,
    pub end_time: Option<Instant>,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            board: Board::new(difficulty),
            start_time: Instant::now(),
            total_pause_duration: Duration::default(),
            start_pause_time: None,
            end_time: None,
        }
    }

    pub fn restart(&mut self) {
        self.board = Board::new(*self.board.get_difficulty());
        self.start_time = Instant::now();
        self.start_pause_time = None;
        self.total_pause_duration = Duration::default();
    }

    pub fn change_difficulty(&mut self, difficulty: Difficulty) {
        self.board = Board::new(difficulty);
        self.start_time = Instant::now();
        self.start_pause_time = None;
        self.total_pause_duration = Duration::default();
    }

    pub fn act(&mut self, cell_action: CellAction) {
        let is_game_running_before_action = self.board.is_game_running();
        match cell_action {
            CellAction::Mark(position) => self.board.toggle_flag(&position),
            CellAction::UnCover(position) => self.board.reveal_cell(&position),
        }

        if is_game_running_before_action && self.board.is_game_ended() {
            self.end_time = Some(Instant::now());
        }
    }

    pub fn get_game_duration(&self) -> Duration {
        if let Some(start_pause_time) = self.start_pause_time {
            let current_pause_duration = Instant::now() - start_pause_time;
            return self.start_time.elapsed() - self.total_pause_duration - current_pause_duration;
        }

        let duration = match self.end_time {
            Some(end_time) => end_time
                .checked_duration_since(self.start_time)
                .unwrap_or(self.start_time.elapsed()),
            None => self.start_time.elapsed(),
        };

        duration - self.total_pause_duration
    }

    pub fn add_pause_duration(&mut self) {
        let Some(start) = self.start_pause_time else {
            return;
        };

        self.total_pause_duration += Instant::now() - start;
        self.start_pause_time = None;
    }
}

pub enum CellAction {
    Mark(Position),
    UnCover(Position),
}
