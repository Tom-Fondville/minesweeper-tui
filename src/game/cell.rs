use crate::game::{board::Board, position::Position};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub state: CellState,
}

#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Hidden,
    Flagged,
    Revealed,
}

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Bomb,
    Numbered(u8),
    Empty,
}

impl Cell {
    pub fn new_bomb() -> Self {
        Self {
            cell_type: CellType::Bomb,
            state: CellState::Hidden,
        }
    }

    pub fn new_numbered(number: u8) -> Self {
        Self {
            cell_type: CellType::Numbered(number),
            state: CellState::Hidden,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            cell_type: CellType::Empty,
            state: CellState::Hidden,
        }
    }

    pub fn is_revealed(&self) -> bool {
        matches!(self.state, CellState::Revealed)
    }

    pub fn is_flagged(&self) -> bool {
        matches!(self.state, CellState::Flagged)
    }

    pub fn can_chord(&self, board: &Board, position: &Position) -> bool {
        let CellType::Numbered(nearby_bomb_count) = self.cell_type else {
            return false;
        };

        let CellState::Revealed = self.state else {
            return false;
        };

        let neigbors = board.get_cell_neighbors(position);
        let flagged_cells_count = neigbors.iter().filter(|c| c.is_flagged()).count();

        nearby_bomb_count == flagged_cells_count as u8
    }

    pub fn reveal(&mut self) {
        if let CellState::Hidden = self.state {
            self.state = CellState::Revealed;
        }
    }
}
