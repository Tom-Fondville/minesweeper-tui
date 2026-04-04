use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::Widget,
};

use crate::{
    game::Game,
    terminal::{app::in_game_view::CursorPositon, ui::board_ui::BoardUi},
};

pub struct InGameUi<'a> {
    game: &'a Game,
    cursor_position: &'a CursorPositon,
}

impl<'a> InGameUi<'a> {
    pub fn new(game: &'a Game, cursor_position: &'a CursorPositon) -> Self {
        Self {
            game,
            cursor_position,
        }
    }
}

impl<'a> Widget for InGameUi<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(area);
        let header = chunks[0];
        let body = chunks[1];
        let footer = chunks[2];

        Line::from(format!(
            " {} - {}",
            self.game.board.get_difficulty().as_string().bold(),
            self.game.board.get_status().as_string(),
        ))
        .centered()
        .render(header, buf);

        BoardUi::new(&self.game.board, self.cursor_position).render(body, buf);

        let footer_text = "minesweeper-tui ".italic();
        let coords_text = Span::from(format!(
            "{}, {} - bomb number {} flags {}, remaining cells {}",
            self.cursor_position.row,
            self.cursor_position.column,
            self.game.board.get_bomb_number(),
            self.game.board.get_flags_count(),
            self.game.board.remaining_cells_count
        ));
        Line::from(vec![footer_text, coords_text]).render(footer, buf);
    }
}
