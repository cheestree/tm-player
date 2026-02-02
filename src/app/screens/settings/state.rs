/// State specific to the settings screen (overlay)
#[derive(Debug, Default)]
pub struct SettingsScreenState {
    selected_index: usize,
}

impl SettingsScreenState {
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn select_next(&mut self, max: usize) {
        if max > 0 {
            self.selected_index = (self.selected_index + 1).min(max - 1);
        }
    }

    pub fn select_previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn reset(&mut self) {
        self.selected_index = 0;
    }
}
