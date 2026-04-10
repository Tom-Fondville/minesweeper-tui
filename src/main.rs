use std::{fs::File, sync::mpsc, thread, time::Duration};

use crossterm::event::{self, Event};
use minesweeper_tui::tui::app::{App, AppEvent};
use ratatui::restore;

fn main() {
    init_log();

    let (event_sender, event_receiver) = mpsc::channel::<AppEvent>();

    let key_input_event_sender = event_sender.clone();
    thread::spawn(move || {
        loop {
            let Event::Key(key_event) = event::read().unwrap() else {
                continue;
            };

            key_input_event_sender
                .send(AppEvent::Input(key_event))
                .unwrap()
        }
    });
    let timer_event_sender = event_sender.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            timer_event_sender.send(AppEvent::Timer).unwrap()
        }
    });

    let mut app = App::new();
    let _ = app.start(&event_receiver);

    restore();
}

fn init_log() {
    let file = File::create("app.log").unwrap();
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
}
