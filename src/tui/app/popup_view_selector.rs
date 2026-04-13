pub struct PopupViewSelector<ViewPopup>
where
    ViewPopup: PartialEq,
{
    pub selected_popup: Option<ViewPopup>,
}

impl<ViewPopup> Default for PopupViewSelector<ViewPopup>
where
    ViewPopup: PartialEq,
{
    fn default() -> Self {
        Self {
            selected_popup: Default::default(),
        }
    }
}

impl<ViewPopup> PopupViewSelector<ViewPopup>
where
    ViewPopup: PartialEq,
{
    pub fn get_selected(&self) -> Option<&ViewPopup> {
        self.selected_popup.as_ref()
    }
    pub fn toggle_popup(&mut self, popup: ViewPopup) {
        if let Some(selected_popup) = &self.selected_popup
            && *selected_popup == popup
        {
            self.selected_popup = None
        }

        self.selected_popup = Some(popup)
    }

    pub fn display_popup(&mut self, popup: ViewPopup) {
        self.selected_popup = Some(popup)
    }

    pub fn hide_popup(&mut self) {
        self.selected_popup = None
    }
}
