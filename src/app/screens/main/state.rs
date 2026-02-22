/// State specific to the main screen
#[derive(Debug, Default)]
pub struct MainScreenState {
    pub selected_track: usize,
    pub selected_playlist: usize,
    pub sorted_by: Option<TrackSort>,
    pub sort_ascending: bool,
}

#[derive(Debug)]
pub enum TrackSort {
    Title,
    Artist,
    Album,
    Duration,
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

    /// Cycle through sort options: Artist↑ → Artist↓ → Title↑ → Title↓ → Album↑ → Album↓ → Duration↑ → Duration↓
    pub fn cycle_sort(&mut self) {
        match (&self.sorted_by, self.sort_ascending) {
            (None, _) => {
                // Start with Artist ascending
                self.sorted_by = Some(TrackSort::Artist);
                self.sort_ascending = true;
            }
            (Some(TrackSort::Artist), true) => {
                // Artist↑ → Artist↓
                self.sort_ascending = false;
            }
            (Some(TrackSort::Artist), false) => {
                // Artist↓ → Title↑
                self.sorted_by = Some(TrackSort::Title);
                self.sort_ascending = true;
            }
            (Some(TrackSort::Title), true) => {
                // Title↑ → Title↓
                self.sort_ascending = false;
            }
            (Some(TrackSort::Title), false) => {
                // Title↓ → Album↑
                self.sorted_by = Some(TrackSort::Album);
                self.sort_ascending = true;
            }
            (Some(TrackSort::Album), true) => {
                // Album↑ → Album↓
                self.sort_ascending = false;
            }
            (Some(TrackSort::Album), false) => {
                // Album↓ → Duration↑
                self.sorted_by = Some(TrackSort::Duration);
                self.sort_ascending = true;
            }
            (Some(TrackSort::Duration), true) => {
                // Duration↑ → Duration↓
                self.sort_ascending = false;
            }
            (Some(TrackSort::Duration), false) => {
                // Duration↓ → Artist↑ (loop back)
                self.sorted_by = None;
                self.sort_ascending = true;
            }
        }
    }
}
