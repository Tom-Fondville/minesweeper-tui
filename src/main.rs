use minesweeper_tui::{
    game::board::{Board, Difficulty},
    terminal::terminal_app::TerminalApp,
};

fn main() {
    println!("Hello, world!");
    let board: Board = Board::new(Difficulty::Easy);
    println!("board: {:?}", board);

    let _ = TerminalApp::start();
}
