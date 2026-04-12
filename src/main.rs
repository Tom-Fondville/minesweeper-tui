use std::fs::File;

use minesweeper_tui::tui::app::App;

fn main() {
    init_log();

    let mut app = App::new();
    let _ = app.start();
}

fn init_log() {
    let file = File::create("app.log").unwrap();
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
}
