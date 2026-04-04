#[derive(Debug, Clone, Copy)]
pub enum Status {
    Running,
    Loosed,
    Won,
}

impl Status {
    pub fn as_string(&self) -> String {
        match self {
            Status::Running => "".to_string(),
            Status::Loosed => "loosed".to_string(),
            Status::Won => "won".to_string(),
        }
    }
}
