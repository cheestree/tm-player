/// State specific to the help screen
#[derive(Debug, Default)]
pub struct HelpScreenState {
    scroll_offset: usize,
}

impl HelpScreenState {

    pub fn scroll_down(&mut self) {
        self.scroll_offset += 1;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}
