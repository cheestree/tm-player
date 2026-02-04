/// State specific to the help screen
#[derive(Debug, Default)]
pub struct HelpScreenState {
    scroll_offset: usize,
}

impl HelpScreenState {
    /// Scroll down the help content by increasing the scroll offset.
    pub fn scroll_down(&mut self) {
        self.scroll_offset += 1;
    }

    /// Scroll up the help content by decreasing the scroll offset.
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}
