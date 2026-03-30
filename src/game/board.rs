#[derive(Debug, Clone)]
pub struct Board {
    rows: u16,
    columns: u16,
    grid: Vec<Vec<Cell>>,
    status: Status,
    difficulty: Difficulty,
}

impl Board {
    pub fn new(difficulty: Difficulty) -> Self {
        let grid: Vec<Vec<Cell>> = vec![
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_numbered(1),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_numbered(1),
                Cell::new_bomb(),
                Cell::new_numbered(1),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_numbered(1),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
        ];

        match difficulty {
            Difficulty::Easy => Self {
                rows: 10,
                columns: 10,
                grid,
                status: Status::Running,
                difficulty,
            },
            Difficulty::Medium => todo!(),
            Difficulty::Hard => todo!(),
            Difficulty::Custom {
                rows_number,
                column_number,
            } => todo!(),
        }
    }

    pub fn get_difficulty(&self) -> &Difficulty {
        &self.difficulty
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

    pub fn reveal_cell(&mut self, position: &Position) {
        let cell = self.get_cell(position);

        let Some(cell) = cell else {
            return;
        };

        if matches!(cell.state, CellState::Revealed | CellState::Flaged) {
            return;
        }

        cell.reveal();
        match cell.cell_type {
            CellType::Bomb => self.status = Status::Loosed,
            CellType::Numbered(_) => {}
            CellType::Empty => {
                if let Some(position) = self.get_cell_top_left_corner_position(position) {
                    self.reveal_cell(&position);
                }
                if let Some(position) = self.get_cell_top_position(position) {
                    self.reveal_cell(&position);
                }
                if let Some(position) = self.get_cell_top_right_corner_position(position) {
                    self.reveal_cell(&position);
                }
                if let Some(position) = self.get_cell_left_position(position) {
                    self.reveal_cell(&position);
                }
                if let Some(position) = self.get_cell_right_position(position) {
                    self.reveal_cell(&position);
                }
                if let Some(position) = self.get_cell_bottom_right_corner_position(position) {
                    self.reveal_cell(&position);
                }
                if let Some(position) = self.get_cell_bottom_position(position) {
                    self.reveal_cell(&position);
                }
                if let Some(position) = self.get_cell_bottom_right_corner_position(position) {
                    self.reveal_cell(&position);
                }
            }
        };
    }

    pub fn get_cell(&mut self, position: &Position) -> Option<&mut Cell> {
        match self.grid.get_mut(position.row as usize) {
            Some(row) => row.get_mut(position.column as usize),
            None => None,
        }
    }

    pub fn get_cell_top_left_corner_position(&mut self, position: &Position) -> Option<Position> {
        if position.row == 0 || self.columns == 0 {
            return None;
        }

        Some(Position::new(position.row - 1, position.column))
    }

    pub fn get_cell_top_position(&mut self, position: &Position) -> Option<Position> {
        if position.row == 0 {
            return None;
        }

        Some(Position::new(position.row - 1, self.columns))
    }

    pub fn get_cell_top_right_corner_position(&mut self, position: &Position) -> Option<Position> {
        if position.row == 0 || position.column == self.columns - 1 {
            return None;
        }

        Some(Position::new(position.row - 1, position.column + 1))
    }

    pub fn get_cell_left_position(&mut self, position: &Position) -> Option<Position> {
        if position.column == 0 {
            return None;
        }

        Some(Position::new(position.row, position.column - 1))
    }

    pub fn get_cell_right_position(&mut self, position: &Position) -> Option<Position> {
        if position.column == self.columns - 1 {
            return None;
        }

        Some(Position::new(position.row, position.column + 1))
    }

    pub fn get_cell_bottom_left_corner_position(
        &mut self,
        position: &Position,
    ) -> Option<Position> {
        if position.row == self.rows - 1 || position.column == 0 {
            return None;
        }

        Some(Position::new(position.row + 1, position.column - 1))
    }

    pub fn get_cell_bottom_position(&mut self, position: &Position) -> Option<Position> {
        if position.row == self.rows {
            return None;
        }

        Some(Position::new(position.row + 1, position.column))
    }

    pub fn get_cell_bottom_right_corner_position(
        &mut self,
        position: &Position,
    ) -> Option<Position> {
        if position.row == self.rows - 1 || position.column == self.columns - 1 {
            return None;
        }

        Some(Position::new(position.row + 1, position.column + 1))
    }
}

pub struct Position {
    pub row: u16,
    pub column: u16,
}

impl Position {
    pub fn new(row: u16, column: u16) -> Self {
        Self { row, column }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub state: CellState,
}

impl Cell {
    pub fn new_bomb() -> Self {
        Self {
            cell_type: CellType::Bomb,
            state: CellState::Hiden,
        }
    }
    pub fn new_numbered(number: u8) -> Self {
        Self {
            cell_type: CellType::Numbered(number),
            state: CellState::Hiden,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            cell_type: CellType::Empty,
            state: CellState::Hiden,
        }
    }

    pub fn reveal(&mut self) {
        if let CellState::Hiden = self.state {
            self.state = CellState::Revealed;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Hiden,
    Flaged,
    Revealed,
}

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Bomb,
    Numbered(u8),
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Running,
    Loosed,
    Won,
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Custom {
        rows_number: u64,
        column_number: u64,
    },
}

impl Difficulty {
    pub fn as_string(&self) -> String {
        match self {
            Difficulty::Easy => "easy".to_string(),
            Difficulty::Medium => "medium".to_string(),
            Difficulty::Hard => "hard".to_string(),
            Difficulty::Custom {
                rows_number,
                column_number,
            } => format!("custom: {}x{}", rows_number, column_number),
        }
    }
}
