use std::time::Duration;

pub mod board;

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub start_date_utc: Duration,
}
