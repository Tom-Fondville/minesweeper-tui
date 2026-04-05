#[derive(Clone, Copy, PartialEq)]
pub enum DifficultyUi {
    Easy,
    Medium,
    Hard,
    Custom,
}

impl DifficultyUi {
    pub fn as_str(&self) -> String {
        let str = match self {
            DifficultyUi::Easy => "Easy",
            DifficultyUi::Medium => "Medium",
            DifficultyUi::Hard => "Hard",
            DifficultyUi::Custom => "Custom",
        };
        str.to_string()
    }
}
