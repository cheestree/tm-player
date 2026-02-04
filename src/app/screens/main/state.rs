
/// State specific to the main screen
#[derive(Debug)]
pub struct MainScreenState {
    pub selected_track: usize,
    pub selected_playlist: usize,
}

impl Default for MainScreenState {
    fn default() -> Self {
        Self {
            selected_track: 0,
            selected_playlist: 0,
        }
    }
}

impl MainScreenState {
    /// Select the next track, up to a maximum index.
    pub fn select_next(&mut self, max: usize) {
        if max > 0 {
            self.selected_track = (self.selected_track + 1).min(max - 1);
        }
    }
    /// Select the previous track, down to a minimum index of 0.
    pub fn select_previous(&mut self) {
        self.selected_track = self.selected_track.saturating_sub(1);
    }

    /// Select the next playlist, up to a maximum index.
    pub fn select_next_playlist(&mut self, max: usize) {
        if max > 0 {
            self.selected_playlist = (self.selected_playlist + 1).min(max - 1);
        }
    }

    /// Select the previous playlist, down to a minimum index of 0.
    pub fn select_previous_playlist(&mut self) {
        self.selected_playlist = self.selected_playlist.saturating_sub(1);
    }
}
