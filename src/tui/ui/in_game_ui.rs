use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget},
};

use crate::{
    game::Game,
    tui::{
        app::in_game_view::CursorPositon,
        ui::{board_ui::BoardUi, helpers::rectangle::centered_rectangle},
    },
};

pub struct InGameUi<'a> {
    game: &'a Game,
    cursor_position: &'a CursorPositon,
    display_help_menu: &'a bool,
    display_confirm_quit_game_popup: &'a bool,
}

impl<'a> InGameUi<'a> {
    pub fn new(
        game: &'a Game,
        cursor_position: &'a CursorPositon,
        display_help_menu: &'a bool,
        display_confirm_quit_game_popup: &'a bool,
    ) -> Self {
        Self {
            game,
            cursor_position,
            display_help_menu,
            display_confirm_quit_game_popup,
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
        if *self.display_help_menu {
            InGameHelpMenuPopupUi::default().render(body, buf);
        }

        if *self.display_confirm_quit_game_popup {
            ConfirmQuitGamePopupUi::default().render(body, buf);
        }

        let footer_layout =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(footer);

        let footer_text = "minesweeper-tui ".italic();
        let coords_text = Span::from(format!(
            "{}, {} - bomb number {} flags {}, remaining cells {}",
            self.cursor_position.row,
            self.cursor_position.column,
            self.game.board.get_bomb_number(),
            self.game.board.get_flags_count(),
            self.game.board.remaining_cells_count
        ));
        Line::from(vec![footer_text, coords_text]).render(footer_layout[0], buf);

        Line::from("\"?\" for help")
            .right_aligned()
            .render(footer_layout[1], buf);
    }
}

#[derive(Default)]
pub struct InGameHelpMenuPopupUi {}

impl Widget for InGameHelpMenuPopupUi {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let popup_area = centered_rectangle(30, 30, area);
        Widget::render(Clear, popup_area, buf);

        let popup_block = Block::default()
            .title("Help popup")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let items = [
            "h     >> go left",
            "j     >> go down",
            "k     >> go up",
            "l     >> go right",
            "f     >> to toggle flag",
            "space >> to reveal cell",
            "enter >> to reveal cell",
            "r     >> to restart",
            "q     >> to quit",
            "?     >> open help menu",
        ];
        let list =
            List::new(items.iter().map(|item| ListItem::new(item.to_string()))).block(popup_block);
        Widget::render(list, popup_area, buf);
    }
}

#[derive(Default)]
pub struct ConfirmQuitGamePopupUi {}

impl ConfirmQuitGamePopupUi {
    pub fn toto(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rectangle(40, 30, area);
        Widget::render(Clear, popup_area, buf);

        let popup_block = Block::default()
            .title(" Confirmation ")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));

        let inner = popup_block.inner(popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Percentage(30),
            ])
            .split(inner);

        // Texte stylé
        let text = vec![
            Line::from(Span::styled(
                "⚠ Quitter la partie",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::raw("Appuie sur ")),
            Line::from(Span::styled(
                "ESC",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw(" à nouveau pour confirmer")),
        ];

        let paragraph = Paragraph::new(text).alignment(Alignment::Center);

        popup_block.render(popup_area, buf);
        paragraph.render(chunks[1], buf);
    }
}
impl Widget for ConfirmQuitGamePopupUi {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let popup_area = centered_rectangle(18, 13, area);
        Widget::render(Clear, popup_area, buf);

        let popup_block = Block::default()
            .title(" Quit Confirmation ")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));

        let text = vec![
            Line::from(Span::styled(
                "⚠ You are about to quit the game",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled(
                    "ESC",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" again to confirm"),
            ]),
        ];

        Paragraph::new(text)
            .block(popup_block)
            .render(popup_area, buf);
    }
}
