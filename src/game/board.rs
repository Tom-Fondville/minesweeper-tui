use std::collections::HashSet;

use rand::RngExt;

use crate::game::{
    cell::{Cell, CellState, CellType},
    difficulty::Difficulty,
    position::Position,
    status::Status,
};

#[derive(Debug, Clone)]
pub struct Board {
    rows: u16,
    columns: u16,
    bomb_number: u16,
    grid: Vec<Vec<Cell>>,
    status: Status,
    difficulty: Difficulty,

    first_cell_has_been_revealed: bool,
    pub remaining_cells_count: u32,
    flags_count: u16,
}

impl Board {
    pub fn new(difficulty: Difficulty) -> Self {
        let rows = difficulty.get_row_number();
        let columns = difficulty.get_column_number();
        Self {
            rows,
            columns,
            bomb_number: difficulty.get_bomb_number(),
            grid: Self::generate_empty_grid(&difficulty),
            status: Status::Running,
            difficulty,
            first_cell_has_been_revealed: false,
            remaining_cells_count: (rows * columns) as u32,
            flags_count: 0,
        }
    }

    pub fn get_difficulty(&self) -> &Difficulty {
        &self.difficulty
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn get_grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }

    pub fn get_rows_count(&self) -> u16 {
        self.rows
    }

    pub fn get_columns_count(&self) -> u16 {
        self.columns
    }

    pub fn get_bomb_number(&self) -> u16 {
        self.bomb_number
    }

    pub fn get_flags_count(&self) -> u16 {
        self.flags_count
    }

    pub fn is_game_running(&self) -> bool {
        matches!(self.status, Status::Running)
    }

    pub fn toggle_flag(&mut self, position: &Position) {
        if !self.is_game_running() {
            return;
        };

        let cell = self.get_cell_mut(position);

        let Some(cell) = cell else {
            return;
        };

        match cell.state {
            CellState::Hidden => {
                cell.state = CellState::Flagged;
                self.flags_count += 1;
            }
            CellState::Flagged => {
                cell.state = CellState::Hidden;
                self.flags_count -= 1;
            }
            CellState::Revealed => {}
        }
    }

    pub fn reveal_cell(&mut self, position: &Position) {
        if !self.is_game_running() {
            return;
        };

        if !self.first_cell_has_been_revealed {
            Self::fill_grid_with_bomb(self, position);
            self.first_cell_has_been_revealed = true;
        }

        let cell = self.get_cell_mut(position);

        let Some(cell) = cell else {
            return;
        };

        if matches!(cell.state, CellState::Revealed)
            && matches!(cell.cell_type, CellType::Numbered(_))
        {
            self.chord(position);
            if self.remaining_cells_count == self.bomb_number as u32 {
                self.status = Status::Won
            }
            return;
        }

        if matches!(cell.state, CellState::Revealed | CellState::Flagged) {
            return;
        }

        match cell.cell_type {
            CellType::Bomb => self.status = Status::Loosed,
            CellType::Numbered(_) => {
                cell.reveal();
                self.remaining_cells_count -= 1;
            }
            CellType::Empty => {
                self.reveal_empty_cell(position);
            }
        };

        if self.remaining_cells_count == self.bomb_number as u32 {
            self.status = Status::Won
        }
    }

    fn chord(&mut self, position: &Position) {
        let cell = self.get_cell(position);
        let Some(cell) = cell else {
            return;
        };

        if !cell.can_chord(self, position) {
            return;
        }

        self.reveal_neighbors(position);
    }

    fn reveal_empty_cell(&mut self, position: &Position) {
        let cell = self.get_cell_mut(position);
        let Some(cell) = cell else {
            return;
        };

        let CellState::Hidden = cell.state else {
            return;
        };

        cell.reveal();
        match cell.cell_type {
            CellType::Bomb => self.status = Status::Loosed,
            CellType::Numbered(_) => {}
            CellType::Empty => {
                let neighbors = self.get_cell_neighbors_positions(position);
                for neighbor in neighbors {
                    self.reveal_empty_cell(&neighbor);
                }
            }
        };

        self.remaining_cells_count -= 1;
    }

    pub fn get_cell_mut(&mut self, position: &Position) -> Option<&mut Cell> {
        match self.grid.get_mut(position.row as usize) {
            Some(row) => row.get_mut(position.column as usize),
            None => None,
        }
    }

    pub fn get_cell(&self, position: &Position) -> Option<&Cell> {
        match self.grid.get(position.row as usize) {
            Some(row) => row.get(position.column as usize),
            None => None,
        }
    }

    pub fn get_cell_top_left_corner_position(&self, position: &Position) -> Option<Position> {
        if position.row == 0 || position.column == 0 {
            return None;
        }

        Some(Position::new(position.row - 1, position.column - 1))
    }

    pub fn get_cell_top_position(&self, position: &Position) -> Option<Position> {
        if position.row == 0 {
            return None;
        }

        Some(Position::new(position.row - 1, position.column))
    }

    pub fn get_cell_top_right_corner_position(&self, position: &Position) -> Option<Position> {
        if position.row == 0 || position.column == self.columns - 1 {
            return None;
        }

        Some(Position::new(position.row - 1, position.column + 1))
    }

    pub fn get_cell_left_position(&self, position: &Position) -> Option<Position> {
        if position.column == 0 {
            return None;
        }

        Some(Position::new(position.row, position.column - 1))
    }

    pub fn get_cell_right_position(&self, position: &Position) -> Option<Position> {
        if position.column == self.columns - 1 {
            return None;
        }

        Some(Position::new(position.row, position.column + 1))
    }

    pub fn get_cell_bottom_left_corner_position(&self, position: &Position) -> Option<Position> {
        if position.row == self.rows - 1 || position.column == 0 {
            return None;
        }

        Some(Position::new(position.row + 1, position.column - 1))
    }

    pub fn get_cell_bottom_position(&self, position: &Position) -> Option<Position> {
        if position.row == self.rows {
            return None;
        }

        Some(Position::new(position.row + 1, position.column))
    }

    pub fn get_cell_bottom_right_corner_position(&self, position: &Position) -> Option<Position> {
        if position.row == self.rows - 1 || position.column == self.columns - 1 {
            return None;
        }

        Some(Position::new(position.row + 1, position.column + 1))
    }

    fn get_cell_neighbors_positions(&self, position: &Position) -> Vec<Position> {
        vec![
            self.get_cell_top_left_corner_position(position),
            self.get_cell_top_position(position),
            self.get_cell_top_right_corner_position(position),
            self.get_cell_left_position(position),
            self.get_cell_right_position(position),
            self.get_cell_bottom_left_corner_position(position),
            self.get_cell_bottom_position(position),
            self.get_cell_bottom_right_corner_position(position),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn reveal_neighbors(&mut self, position: &Position) {
        let positions = self.get_cell_neighbors_positions(position);
        for position in positions {
            let Some(cell) = self.get_cell_mut(&position) else {
                continue;
            };

            if cell.is_flagged() || cell.is_revealed() {
                continue;
            }

            match cell.cell_type {
                CellType::Bomb => self.status = Status::Loosed,
                CellType::Numbered(_) => {
                    cell.reveal();
                    self.remaining_cells_count -= 1;
                }
                CellType::Empty => {
                    self.reveal_empty_cell(&position);
                }
            };
        }
    }

    pub fn get_cell_neighbors(&self, position: &Position) -> Vec<&Cell> {
        let mut neighbors = Vec::new();
        for position in self.get_cell_neighbors_positions(position) {
            if let Some(cell) = self.get_cell(&position) {
                neighbors.push(cell);
            }
        }
        neighbors
    }

    fn generate_empty_grid(difficulty: &Difficulty) -> Vec<Vec<Cell>> {
        let mut grid = Vec::new();
        for _ in 0..difficulty.get_row_number() {
            let mut row = Vec::new();
            for _ in 0..difficulty.get_column_number() {
                row.push(Cell::new_empty());
            }
            grid.push(row);
        }
        grid
    }

    pub fn fill_grid_with_bomb(&mut self, first_safe_cell_position: &Position) {
        let mut bombs_position: HashSet<Position> = HashSet::new();
        let mut excluded_positions: HashSet<Position> =
            HashSet::from([first_safe_cell_position.clone()]);

        for _ in 0..self.difficulty.get_bomb_number() {
            let bomb_position =
                Self::generate_random_position(&self.difficulty, &excluded_positions);
            bombs_position.insert(bomb_position.clone());
            excluded_positions.insert(bomb_position.clone());
        }
        log::warn!("bomb_position: {:?}", bombs_position);

        for bomb_possition in bombs_position {
            if let Some(cell) = self.get_cell_mut(&bomb_possition) {
                cell.cell_type = CellType::Bomb
            }
        }

        for row in 0..self.difficulty.get_row_number() {
            for column in 0..self.difficulty.get_column_number() {
                let position = Position::new(row, column);
                let Some(cell) = self.get_cell(&position) else {
                    continue;
                };

                let CellType::Empty = cell.cell_type else {
                    continue;
                };

                let neighbors = self.get_cell_neighbors(&position);
                let number = Self::compute_cell_number(&neighbors);

                let Some(cell) = self.get_cell_mut(&position) else {
                    continue;
                };
                match number {
                    0 => cell.cell_type = CellType::Empty,
                    _ => cell.cell_type = CellType::Numbered(number),
                }
            }
        }
    }

    fn compute_cell_number(neighbors: &Vec<&Cell>) -> u8 {
        let mut number = 0;
        for neighbor in neighbors {
            if let CellType::Bomb = neighbor.cell_type {
                number += 1;
            }
        }

        number
    }

    fn generate_random_position(
        difficulty: &Difficulty,
        excluded_position: &HashSet<Position>,
    ) -> Position {
        loop {
            let row = rand::rng().random_range(0..difficulty.get_row_number());
            let column = rand::rng().random_range(0..difficulty.get_column_number());
            let position = Position::new(row, column);
            if !excluded_position.contains(&position) {
                break position;
            }
        }
    }
}
