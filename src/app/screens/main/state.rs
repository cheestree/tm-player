use ratatui::widgets::ListState;

/// State specific to the main screen
#[derive(Debug)]
pub struct MainScreenState {
    selected_track: usize,
    list_state: ListState,
}

impl Default for MainScreenState {
    fn default() -> Self {
        Self {
            selected_track: 0,
            list_state: ListState::default(),
        }
    }
}

impl MainScreenState {
    pub fn selected_track(&self) -> usize {
        self.selected_track
    }

    pub fn select_next(&mut self, max: usize) {
        if max > 0 {
            self.selected_track = (self.selected_track + 1).min(max - 1);
        }
    }

    pub fn select_previous(&mut self) {
        self.selected_track = self.selected_track.saturating_sub(1);
    }
}
