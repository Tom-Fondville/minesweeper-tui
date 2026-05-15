use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{
    game::{
        board::Board,
        cell::{Cell, CellState, CellType},
        status::Status,
    },
    tui::app::in_game_view::CursorPositon,
};

const CELL_WIDTH: u16 = 7;
const CELL_HEIGHT: u16 = 3;

pub struct BoardUi<'a> {
    board: &'a Board,
    cursor_position: &'a CursorPositon,
}

impl<'a> BoardUi<'a> {
    pub fn new(board: &'a Board, cursor_position: &'a CursorPositon) -> Self {
        BoardUi {
            board,
            cursor_position,
        }
    }
}

impl<'a> BoardUi<'a> {
    fn has_to_render_cursor(&self, row: usize, column: usize) -> bool {
        if !self.board.is_game_running() {
            return false;
        }

        if row as u16 != self.cursor_position.row {
            return false;
        }

        if column as u16 != self.cursor_position.column {
            return false;
        }

        true
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
        let grid_width = (2 + CELL_WIDTH) * collumns;
        let grid_height = (1 + CELL_HEIGHT) * rows;

        if area.width <= grid_width || area.height <= grid_height {
            Line::from(
                "your terminal is to small to display the grid, please rezise your terminal",
            )
            .centered()
            .fg(Color::Yellow)
            .bold()
            .bg(Color::LightRed)
            .render(area, buf);
            return;
        }

        let grid_area = Rect {
            x: area.x + (area.width - grid_width) / 2,
            y: area.y + (area.height - grid_height) / 2,
            width: grid_width,
            height: grid_height,
        };

        // here to debug the size of the grid
        // frame.render_widget(Block::default().bg(Color::DarkGray), grid_area);
        // return;

        // let row_chunks = Layout::vertical(vec![Constraint::Length(CELL_HEIGHT); rows as usize]).split(grid_area);
        // for (row, cells_row) in grid.iter().enumerate() {
        //     let col_chunks =
        //         Layout::horizontal(vec![Constraint::Length(CELL_WIDTH); collumns as usize])
        //             .split(row_chunks[row]);

        for (row, cells_row) in grid.iter().enumerate() {
            for (column, cell) in cells_row.iter().enumerate() {
                let cell_area = Rect {
                    x: grid_area.x + column as u16 * (CELL_WIDTH + 2),
                    y: grid_area.y + row as u16 * (CELL_HEIGHT + 1),
                    width: CELL_WIDTH,
                    height: CELL_HEIGHT,
                };

                let display_cursor = self.has_to_render_cursor(row, column);
                render_cell(
                    cell,
                    display_cursor,
                    self.board.get_status(),
                    cell_area,
                    buf,
                );
            }
        }
    }
}

fn render_cell(
    cell: &Cell,
    display_cursor: bool,
    board_status: &Status,
    area: Rect,
    buf: &mut Buffer,
) {
    let mut style = match board_status {
        Status::Running => match cell.state {
            CellState::Hidden => Style::default().bg(Color::Gray),
            CellState::Flagged => Style::default().bg(Color::Gray).fg(Color::Black),
            CellState::Revealed => match cell.cell_type {
                CellType::Bomb => Style::default().bg(Color::Red).fg(Color::White),
                CellType::Numbered(_) => Style::default().bg(Color::DarkGray).fg(Color::Yellow),
                CellType::Empty => Style::default().bg(Color::Black),
            },
        },
        _ => match cell.cell_type {
            CellType::Bomb => Style::default().bg(Color::Red).fg(Color::White),
            CellType::Numbered(_) => Style::default().bg(Color::DarkGray).fg(Color::Yellow),
            CellType::Empty => Style::default().bg(Color::Black),
        },
    };

    if display_cursor {
        style = style.patch(Style::default().reversed());
    }

    let block = Block::new().style(style).padding(Padding::top(1));

    match board_status {
        Status::Running => match cell.state {
            CellState::Hidden => block.render(area, buf),
            CellState::Flagged => Paragraph::new("F")
                .block(block)
                .centered()
                .render(area, buf),
            CellState::Revealed => match cell.cell_type {
                CellType::Bomb => Paragraph::new("*")
                    .block(block)
                    .centered()
                    .render(area, buf),
                CellType::Numbered(number) => Paragraph::new(number.to_string())
                    .block(block)
                    .centered()
                    .render(area, buf),
                CellType::Empty => block.render(area, buf),
            },
        },
        _ => match cell.state {
            CellState::Flagged => match cell.cell_type {
                CellType::Numbered(number) => Paragraph::new(format!("{}   F", number))
                    .block(block)
                    .centered()
                    .render(area, buf),
                _ => Paragraph::new("F")
                    .block(block)
                    .centered()
                    .render(area, buf),
            },
            _ => match cell.cell_type {
                CellType::Bomb => Paragraph::new("*")
                    .block(block)
                    .centered()
                    .render(area, buf),
                CellType::Numbered(number) => Paragraph::new(number.to_string())
                    .block(block)
                    .centered()
                    .render(area, buf),
                CellType::Empty => block.render(area, buf),
            },
        },
    }
}
