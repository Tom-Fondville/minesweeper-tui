use std::ptr::read;

use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::Block,
};

use crate::game::{
    app::{App, AppState},
    board::{Board, CellType},
};

pub struct TerminalInGameState {}
impl TerminalInGameState {
    pub fn draw(terminal: &mut DefaultTerminal, board: &Board) {
        let _ = terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(frame.area());
            let header = chunks[0];
            let body = chunks[1];
            let footer = chunks[2];

            let title =
                Line::from(format!(" {} ", board.get_difficulty().as_string().bold())).centered();
            frame.render_widget(title, header);

            let board = board.get_grid();
            let row_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(1); board.len()])
                .split(body);

            for (y, cells_row) in board.iter().enumerate() {
                let col_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Length(2); cells_row.len()])
                    .split(row_chunks[y]);

                for (x, cell) in cells_row.iter().enumerate() {
                    let block_cell = match cell.cell_type {
                        CellType::Bomb => Block::default()
                            .title("")
                            .style(Style::default().bg(Color::Red).red()),
                        CellType::Numbered(number) => Block::default()
                            .title(number.to_string())
                            .style(Style::default().bg(Color::Gray).black().bold()),
                        CellType::Empty => Block::default()
                            .title("")
                            .style(Style::default().bg(Color::White).white()),
                    };

                    frame.render_widget(block_cell, col_chunks[x]);
                }
            }
            let footer_text = Line::from("minesweeper-tui".italic());
            frame.render_widget(footer_text, footer);
        });
    }

    pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
        match key_event.kind {
            event::KeyEventKind::Press => {
                if let KeyCode::Char('q') = key_event.code {
                    app.change_current_state(AppState::Exiting(false));
                }
            }
            event::KeyEventKind::Repeat => (),
            event::KeyEventKind::Release => (),
        }
    }
}
