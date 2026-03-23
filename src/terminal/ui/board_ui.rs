use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{
    game::board::{Board, Cell, CellType},
    terminal::terminal_state::terminal_in_game_state::CursorPositon,
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

        for (row, cells_row) in grid.iter().enumerate() {
            let col_chunks =
                Layout::horizontal(vec![Constraint::Length(CELL_WIDTH); collumns as usize])
                    .split(row_chunks[row]);

            for (column, cell) in cells_row.iter().enumerate() {
                if row as u16 == self.cursor_position.row
                    && column as u16 == self.cursor_position.column
                {
                    render_cell(cell, true, col_chunks[column], buf);
                } else {
                    render_cell(cell, false, col_chunks[column], buf);
                }
            }
        }
    }
}

fn render_cell(cell: &Cell, display_cursor: bool, area: Rect, buf: &mut Buffer) {
    let mut style = match cell.cell_type {
        CellType::Bomb => Style::default().bg(Color::Red).fg(Color::White),
        CellType::Numbered(_) => Style::default().bg(Color::DarkGray).fg(Color::Yellow),
        CellType::Empty => Style::default().bg(Color::Black),
    };

    if display_cursor {
        style = style.patch(Style::default().reversed());
    }

    let block = Block::new().style(style).padding(Padding::top(1));

    match cell.cell_type {
        CellType::Bomb => Paragraph::new("*")
            .block(block)
            .centered()
            .render(area, buf),
        CellType::Numbered(number) => Paragraph::new(number.to_string())
            .block(block)
            .centered()
            .render(area, buf),
        CellType::Empty => block.render(area, buf),
    };
}
