use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
    },
};

use crate::tui::{
    app::main_menu_view::{
        CustomDifficultyInputs, MainMenuViewPopup, SelectedCustomDifficultyInput,
    },
    ui::{
        difficulty_ui::DifficultyUi, helpers::rectangle::centered_rectangle_exact,
        in_game_ui::ExitAppConfirmationPopupUi,
    },
};

pub struct MainMenuUi<'a> {
    difficulties: &'a [DifficultyUi; 4],
    custom_difficulty_inputs: &'a CustomDifficultyInputs,
    selected_input: &'a SelectedCustomDifficultyInput,
    displayed_popup: Option<&'a MainMenuViewPopup>,
}

impl<'a> MainMenuUi<'a> {
    pub fn new(
        difficulties: &'a [DifficultyUi; 4],
        custom_difficulty_inputs: &'a CustomDifficultyInputs,
        selected_input: &'a SelectedCustomDifficultyInput,
        displayed_popup: Option<&'a MainMenuViewPopup>,
    ) -> Self {
        Self {
            difficulties,
            custom_difficulty_inputs,
            selected_input,
            displayed_popup,
        }
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

        let body_layout =
            Layout::vertical(vec![Constraint::Max(6), Constraint::Min(3)]).split(body);

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

        StatefulWidget::render(list, body_layout[0], buf, state);
        if self.get_selected_difficulty(state) == DifficultyUi::Custom {
            CustomDifficultyInputsUi::new(self.custom_difficulty_inputs, self.selected_input)
                .render(body_layout[1], buf);
        }

        if let Some(displayed_popup) = self.displayed_popup {
            match displayed_popup {
                MainMenuViewPopup::HelpMenu => MainMenuHelpMenuPopupUi::default().render(body, buf),
                MainMenuViewPopup::ExitAppConfirmation => {
                    ExitAppConfirmationPopupUi::default().render(body, buf)
                }
            }
        };

        let footer_layout =
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(footer);

        Line::from(format!(
            "selected: {}",
            self.get_selected_difficulty(state).as_str()
        ))
        .render(footer_layout[0], buf);
        Line::from("\"?\" for help")
            .right_aligned()
            .render(footer_layout[1], buf);
    }
}

pub struct CustomDifficultyInputsUi<'a> {
    rows: &'a str,
    columns: &'a str,
    bombs: &'a str,
    selected_input: &'a SelectedCustomDifficultyInput,
}

impl<'a> CustomDifficultyInputsUi<'a> {
    pub fn new(
        custom_difficulty_inputs: &'a CustomDifficultyInputs,
        selected_input: &'a SelectedCustomDifficultyInput,
    ) -> Self {
        Self {
            rows: &custom_difficulty_inputs.rows,
            columns: &custom_difficulty_inputs.colomns,
            bombs: &custom_difficulty_inputs.bombs,
            selected_input,
        }
    }
}

impl<'a> Widget for CustomDifficultyInputsUi<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::horizontal(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(area);

        let mut rows_paragraph =
            Paragraph::new(self.rows).block(Block::default().title("rows").borders(Borders::ALL));

        let mut columns_paragraph = Paragraph::new(self.columns)
            .block(Block::default().title("columns").borders(Borders::ALL));

        let mut bombs_paragraph =
            Paragraph::new(self.bombs).block(Block::default().title("bombs").borders(Borders::ALL));

        let selected_style = Style::default().bg(Color::Cyan);
        match self.selected_input {
            SelectedCustomDifficultyInput::None => (),
            SelectedCustomDifficultyInput::Rows => {
                rows_paragraph = rows_paragraph.style(selected_style)
            }
            SelectedCustomDifficultyInput::Colomns => {
                columns_paragraph = columns_paragraph.style(selected_style);
            }
            SelectedCustomDifficultyInput::Bombs => {
                bombs_paragraph = bombs_paragraph.style(selected_style)
            }
        }

        rows_paragraph.render(
            Layout::vertical(vec![Constraint::Max(3)]).split(chunks[0])[0],
            buf,
        );
        columns_paragraph.render(
            Layout::vertical(vec![Constraint::Max(3)]).split(chunks[1])[0],
            buf,
        );
        bombs_paragraph.render(
            Layout::vertical(vec![Constraint::Max(3)]).split(chunks[2])[0],
            buf,
        );
    }
}

#[derive(Default)]
pub struct MainMenuHelpMenuPopupUi {}
impl Widget for MainMenuHelpMenuPopupUi {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let popup_block = Block::default()
            .title("Help popup")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let items = [
            "j     >> go down",
            "k     >> go up",
            "tab   >> cycle through inputs for custom difficulty",
            "enter >> start new game",
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
