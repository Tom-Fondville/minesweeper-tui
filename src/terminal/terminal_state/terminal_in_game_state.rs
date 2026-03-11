use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::game::{
    app::{App, AppState},
    board::{Board, Cell, CellType},
};

pub struct TerminalInGameState {}
impl TerminalInGameState {
    pub fn render_cell(frame: &mut Frame, cell: &Cell) {
        let cell_area = Rect::new(x, y, width, height)
    }

    pub fn draw_grid(frame: &mut Frame, area: Rect, board: &Board) {
        let grid = board.get_grid();

        let rows = board.get_rows_count();
        let collumns = board.get_columns_count();
        let cell_size = (area.height / rows).min(area.width / collumns);
        let grid_width = cell_size * collumns;
        let grid_height = cell_size * rows;
        println!("width {}, height {}", grid_width, grid_height);

        let grid_area = Rect {
            x: area.x + (area.width - grid_width) / 2,
            y: area.y + (area.height - grid_height) / 2,
            width: 10,
            height: 10,
        };

        frame.render_widget(Block::default().bg(Color::Red), grid_area);
        return;

        let row_chunks =
            Layout::vertical(vec![Constraint::Length(cell_size); rows as usize]).split(grid_area);

        for (y, cells_row) in grid.iter().enumerate() {
            let col_chunks =
                Layout::horizontal(vec![Constraint::Length(cell_size); collumns as usize])
                    .split(row_chunks[y]);

            for (x, cell) in cells_row.iter().enumerate() {
                let block_cell = match cell.cell_type {
                    CellType::Bomb => Paragraph::new("*")
                        .style(Style::default().bg(Color::Red).fg(Color::White))
                        .centered(),
                    CellType::Numbered(number) => Paragraph::new(number.to_string())
                        .style(Style::default().bg(Color::DarkGray).fg(Color::Yellow))
                        .centered(),
                    CellType::Empty => Paragraph::new("")
                        .style(Style::default().bg(Color::Gray))
                        .centered(),
                };

                frame.render_widget(
                    // block_cell.block(Block::default().borders(Borders::ALL)),
                    block_cell,
                    col_chunks[x],
                );
            }
        }
    }

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

            TerminalInGameState::draw_grid(frame, body, board);

            // let row_chunks = Layout::default()
            //     .direction(Direction::Vertical)
            //     .constraints(vec![Constraint::Ratio(1, grid.len() as u32); grid.len()])
            //     .split(body);
            //
            // for (y, cells_row) in grid.iter().enumerate() {
            //     let col_chunks = Layout::default()
            //         .direction(Direction::Horizontal)
            //         .constraints(vec![
            //             Constraint::Ratio(1, cells_row.len() as u32);
            //             cells_row.len()
            //         ])
            //         .split(row_chunks[y]);
            //
            //     for (x, cell) in cells_row.iter().enumerate() {
            //         let block_cell = match cell.cell_type {
            //             CellType::Bomb => Paragraph::new("*")
            //                 .style(Style::default().bg(Color::Red).fg(Color::White))
            //                 .centered(),
            //             CellType::Numbered(number) => Paragraph::new(number.to_string())
            //                 .style(Style::default().bg(Color::DarkGray).fg(Color::Yellow))
            //                 .centered(),
            //             CellType::Empty => Paragraph::new("")
            //                 .style(Style::default().bg(Color::Gray))
            //                 .centered(),
            //         };
            //
            //         frame.render_widget(block_cell, col_chunks[x]);
            //     }
            // }

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
