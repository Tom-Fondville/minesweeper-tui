use crate::{
    game::board::{Board, Difficulty},
    terminal_app::TerminalApp,
};

pub mod app;
pub mod game;
pub mod terminal_app;
fn main() {
    println!("Hello, world!");
    let board: Board = Board::new(Difficulty::Easy);
    println!("board: {:?}", board);

    let _ = TerminalApp::start();
}
