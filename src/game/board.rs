#[derive(Debug, Clone)]
pub struct Board {
    rows: u16,
    column: u16,
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
                column: 10,
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
        self.column
    }

    pub fn reveal_cell(&self, position: &Position) {
        let cell = match self.grid.get(position.row) {
            Some(row) => row.get(position.column),
            None => None,
        };

        // match cell {
        //     Some(cell) => cell.reveal(),
        //     None => {}
        // }
        //
    }
}

pub struct Position {
    pub row: usize,
    pub column: usize,
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
