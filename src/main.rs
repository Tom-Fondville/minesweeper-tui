use minesweeper_tui::game::board::{Board, Difficulty};

fn main() {
    println!("Hello, world!");
    let board: Board = Board::new(Difficulty::Easy);
    println!("board: {:?}", board);
}
