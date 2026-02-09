/// State specific to the settings screen (overlay)
#[derive(Debug, Default)]
pub struct SettingsScreenState {
    pub selected_index: usize,
}

impl SettingsScreenState {
    /// Select the next item in the settings list, up to a maximum index.
    pub fn select_next(&mut self, max: usize) {
        if max > 0 {
            self.selected_index = (self.selected_index + 1).min(max - 1);
        }
    }

    /// Select the previous item in the settings list, down to a minimum index of 0.
    pub fn select_previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    /// Reset the selected index to 0.
    pub fn reset(&mut self) {
        self.selected_index = 0;
    }
}
