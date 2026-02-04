use crate::app::app::App;
use ratatui::prelude::Widget;

pub fn render(area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, _app: &App) {
    let block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .title("Help Screen - Press 'Esc' to return");
    block.render(area, buf);
}
