#[derive(Debug)]
pub struct Board {
    rows: u64,
    column: u64,
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
            ],
            vec![
                Cell::new_empty(),
                Cell::new_numbered(1),
                Cell::new_bomb(),
                Cell::new_numbered(1),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_numbered(1),
                Cell::new_empty(),
            ],
            vec![
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ],
        ];

        match difficulty {
            Difficulty::Easy => {
                return Self {
                    rows: 10,
                    column: 10,
                    grid,
                    status: Status::Running,
                    difficulty,
                };
            }
            Difficulty::Medium => todo!(),
            Difficulty::Hard => todo!(),
            Difficulty::Custom {
                rows_number,
                column_number,
            } => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    pub cell_type: CellType,
    pub is_flaged: bool,
}

impl Cell {
    pub fn new_bomb() -> Self {
        Self {
            cell_type: CellType::Bomb,
            is_flaged: false,
        }
    }
    pub fn new_numbered(number: u8) -> Self {
        Self {
            cell_type: CellType::Numbered(number),
            is_flaged: false,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            cell_type: CellType::Empty,
            is_flaged: false,
        }
    }
}

#[derive(Debug)]
pub enum CellType {
    Bomb,
    Numbered(u8),
    Empty,
}

#[derive(Debug)]
pub enum Status {
    Running,
    Loosed,
    Won,
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Custom {
        rows_number: u64,
        column_number: u64,
    },
}
