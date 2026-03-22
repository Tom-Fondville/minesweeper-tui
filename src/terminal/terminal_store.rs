use crate::game::board::{Board, Difficulty};

pub struct TerminalStore {
    board: Option<Board>,
}

impl TerminalStore {
    pub fn get_current_board(&mut self, difficulty: Difficulty) -> &mut Board {
        self.board.get_or_insert(Board::new(difficulty))
    }
}
