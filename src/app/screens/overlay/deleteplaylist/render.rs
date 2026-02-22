use crate::app::app::App;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Modifier, Span, Style, Stylize, Widget};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render(area: Rect, buf: &mut Buffer, _app: &App, title: &str, description: &Option<String>) {
    let popup_width = area.width.min(60);
    let popup_height = if description.is_some() { 9 } else { 7 };
    let popup_x = area.x + (area.width - popup_width) / 2;
    let popup_y = area.y + (area.height - popup_height) / 2;
    let popup_area = Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    };

    // Clear the popup area with terminal default color to cover content underneath
    let background = Block::default().bg(Color::Reset);
    background.render(popup_area, buf);

    let block = Block::default()
        .borders(Borders::ALL)
        .bg(Color::Reset)
        .title(title)
        .border_style(Style::default().fg(Color::Yellow));
    block.render(popup_area, buf);

    let inner_area = Rect {
        x: popup_area.x + 1,
        y: popup_area.y + 1,
        width: popup_area.width.saturating_sub(2),
        height: popup_area.height.saturating_sub(2),
    };

    // Split inner area for content
    let chunks = Layout::vertical([
        Constraint::Min(1),    // Description/spacing
        Constraint::Length(1), // Empty line
        Constraint::Length(1), // Confirmation prompt
    ])
    .split(inner_area);

    // Render description if provided
    if let Some(desc) = description {
        let desc_paragraph = Paragraph::new(desc.as_str())
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));
        desc_paragraph.render(chunks[0], buf);
    }

    // Render confirmation prompt
    let prompt = Line::from(vec![
        Span::raw("Press "),
        Span::styled(
            "Y",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" to confirm or "),
        Span::styled(
            "N",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw("/"),
        Span::styled(
            "ESC",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" to cancel"),
    ]);

    Paragraph::new(prompt).render(chunks[2], buf);
}
