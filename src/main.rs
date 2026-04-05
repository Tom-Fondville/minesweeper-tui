use std::fs::File;

use minesweeper_tui::tui::app::App;
use ratatui::restore;

fn main() {
    init_log();

    let mut app = App::new();
    let _ = app.start();

    restore();
}

fn init_log() {
    let file = File::create("app.log").unwrap();
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
}
