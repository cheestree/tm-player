use crate::app::app::App;
use crate::app::screens::utils::centered_rect;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style, Widget};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

/// Render the rename playlist overlay
pub fn render(
    area: Rect,
    buf: &mut Buffer,
    _app: &App,
    _playlist_index: usize,
    current_name: &str,
) {
    // Create a centered popup
    let popup_area = centered_rect(60, 30, area);

    // Clear the area
    Clear.render(popup_area, buf);

    let text = vec![
        Line::from(""),
        Line::from(format!("New name: {}", current_name)),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Rename Playlist")
                .title_bottom(Line::from(" Type name | Enter: Save | Esc: Cancel ").centered()),
        )
        .style(Style::default().fg(Color::White));

    paragraph.render(popup_area, buf);
}
