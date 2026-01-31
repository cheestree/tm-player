use ratatui::prelude::Widget;

pub fn render(area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, app: &crate::app::app::App) {
    let block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .title("Help Screen - Press 'h' to return");
    block.render(area, buf);
}