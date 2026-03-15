use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::game::{
    app::{App, AppState},
    board::{Board, Cell, CellType},
};

const CELL_WIDTH: u16 = 7;
const CELL_HEIGHT: u16 = 3;

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

            frame.render_widget(BoardUi::new(board), body);

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

pub struct BoardUi<'a> {
    board: &'a Board,
}

impl<'a> BoardUi<'a> {
    pub fn new(board: &'a Board) -> Self {
        BoardUi { board }
    }
}

impl<'a> Widget for BoardUi<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let grid = self.board.get_grid();

        let rows = self.board.get_rows_count();
        let collumns = self.board.get_columns_count();
        let grid_width = CELL_WIDTH * collumns;
        let grid_height = CELL_HEIGHT * rows;

        let grid_area = Rect {
            x: area.x + (area.width - grid_width) / 2,
            y: area.y + (area.height - grid_height) / 2,
            width: grid_width,
            height: grid_height,
        };

        // here to debug the size of the grid
        // frame.render_widget(Block::default().bg(Color::DarkGray), grid_area);
        // return;

        let row_chunks =
            Layout::vertical(vec![Constraint::Length(CELL_HEIGHT); rows as usize]).split(grid_area);

        for (y, cells_row) in grid.iter().enumerate() {
            let col_chunks =
                Layout::horizontal(vec![Constraint::Length(CELL_WIDTH); collumns as usize])
                    .split(row_chunks[y]);

            for (x, cell) in cells_row.iter().enumerate() {
                render_cell(cell, col_chunks[x], buf);
            }
        }
    }
}

fn render_cell(cell: &Cell, area: Rect, buf: &mut Buffer) {
    match cell.cell_type {
        CellType::Bomb => Paragraph::new("*")
            .block(
                Block::new()
                    .style(Style::default().bg(Color::Red).fg(Color::White))
                    .padding(Padding::top(1)),
            )
            .centered()
            .render(area, buf),
        CellType::Numbered(number) => Paragraph::new(number.to_string())
            .block(
                Block::new()
                    .style(Style::default().bg(Color::DarkGray).fg(Color::Yellow))
                    .padding(Padding::top(1)),
            )
            .centered()
            .render(area, buf),
        CellType::Empty => Block::new()
            .on_black()
            // .borders(Borders::all())
            .render(area, buf),
    };
}
