use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget},
};

use crate::{
    game::{Game, difficulty::Difficulty, status::Status},
    tui::{
        app::in_game_view::CursorPositon,
        ui::{board_ui::BoardUi, helpers::rectangle::centered_rectangle_exact},
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

        if !self.game.board.is_game_running() {
            GameEndedPopupUi::new(
                self.game.board.get_status(),
                self.game.board.get_difficulty(),
            )
            .render(body, buf);
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

        let width = items
            .iter()
            .map(|line| line.chars().count() as u16)
            .max()
            .unwrap_or(0)
            + 4;
        let height = list.len() as u16 + 2;
        let popup_area = centered_rectangle_exact(width, height, area);
        Widget::render(Clear, popup_area, buf);

        Widget::render(list, popup_area, buf);
    }
}

#[derive(Default)]
pub struct ConfirmQuitGamePopupUi {}
impl Widget for ConfirmQuitGamePopupUi {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
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
                    "ENTER",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to confirm"),
            ]),
        ];

        let width = text
            .iter()
            .map(|line| line.width() as u16)
            .max()
            .unwrap_or(0)
            + 4;
        let height = text.len() as u16 + 2;
        let popup_area = centered_rectangle_exact(width, height, area);
        Widget::render(Clear, popup_area, buf);

        Paragraph::new(text)
            .block(popup_block)
            .render(popup_area, buf);
    }
}

pub struct GameEndedPopupUi<'a> {
    game_status: &'a Status,
    difficulty: &'a Difficulty,
}

impl<'a> GameEndedPopupUi<'a> {
    fn new(game_status: &'a Status, difficulty: &'a Difficulty) -> Self {
        Self {
            game_status,
            difficulty,
        }
    }
}

impl<'a> Widget for GameEndedPopupUi<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let popup_block = Block::default()
            .title(" Game ended ")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));

        let mut text = vec![
            Line::from(Span::styled(
                "Game ended in X amout of time (comming soong)",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        match self.game_status {
            Status::Running => unreachable!(),
            Status::Won => text.push(Line::from(vec![
                Span::raw("You "),
                Span::styled(
                    "WON",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    " in difficulty {} !!!",
                    self.difficulty.as_string()
                )),
            ])),
            Status::Loosed => text.push(Line::from(vec![
                Span::raw("You "),
                Span::styled(
                    "LOOSED",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(", you can always try again"),
            ])),
        }

        text.push(Line::from(vec![
            Span::raw("Press "),
            Span::styled(
                "r",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" to play again"),
        ]));

        let width = text
            .iter()
            .map(|line| line.width() as u16)
            .max()
            .unwrap_or(0)
            + 4;
        let height = text.len() as u16 + 2;
        let popup_area = centered_rectangle_exact(width, height, area);
        Widget::render(Clear, popup_area, buf);

        Paragraph::new(text)
            .block(popup_block)
            .render(popup_area, buf);
    }
}
