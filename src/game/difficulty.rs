#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Custom {
        rows_number: u16,
        column_number: u16,
        bomb_number: u16,
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
                bomb_number: _,
            } => format!("custom: {}x{}", rows_number, column_number),
        }
    }

    pub fn get_row_number(&self) -> u16 {
        match self {
            Difficulty::Easy => 9,
            Difficulty::Medium => 16,
            Difficulty::Hard => 30,
            Difficulty::Custom {
                rows_number,
                column_number: _,
                bomb_number: _,
            } => *rows_number,
        }
    }

    pub fn get_column_number(&self) -> u16 {
        match self {
            Difficulty::Easy => 9,
            Difficulty::Medium => 16,
            Difficulty::Hard => 16,
            Difficulty::Custom {
                rows_number,
                column_number: _,
                bomb_number: _,
            } => *rows_number,
        }
    }

    pub fn get_bomb_number(&self) -> u16 {
        match self {
            Difficulty::Easy => 10,
            Difficulty::Medium => 40,
            Difficulty::Hard => 99,
            Difficulty::Custom {
                rows_number,
                column_number: _,
                bomb_number: _,
            } => *rows_number,
        }
    }
}
