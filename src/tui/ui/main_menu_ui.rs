use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::tui::ui::difficulty_ui::DifficultyUi;

pub struct MainMenuUi<'a> {
    difficulties: &'a [DifficultyUi; 4],
}

impl<'a> MainMenuUi<'a> {
    pub fn new(difficulties: &'a [DifficultyUi; 4]) -> Self {
        Self { difficulties }
    }

    pub fn get_selected_difficulty(&self, state: &mut ListState) -> DifficultyUi {
        self.difficulties[state.selected().unwrap_or(0)]
    }
}

impl<'a> StatefulWidget for MainMenuUi<'a> {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let chunks = ratatui::layout::Layout::default()
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

        Line::from("Press enter to play. Press q to quit").render(header, buf);

        let mut difficulty_items = Vec::<ListItem>::new();
        for difficulty in self.difficulties {
            difficulty_items.push(ListItem::new(difficulty.as_str()));
        }

        let list = List::new(difficulty_items)
            .block(
                Block::default()
                    .title("Select Difficulty")
                    .borders(Borders::ALL),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        StatefulWidget::render(list, body, buf, state);

        Line::from(format!(
            "selected: {}",
            self.get_selected_difficulty(state).as_str()
        ))
        .render(footer, buf);
    }
}
