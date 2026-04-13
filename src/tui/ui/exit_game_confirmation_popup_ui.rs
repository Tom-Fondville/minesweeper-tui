use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::tui::ui::helpers::rectangle::centered_rectangle_exact;

#[derive(Default)]
pub struct ExitGameConfirmationPopupUi {}
impl Widget for ExitGameConfirmationPopupUi {
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
