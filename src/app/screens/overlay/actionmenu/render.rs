use crate::app::app::App;
use crate::app::screens::overlay::actionmenu::common::get_available_actions;
use crate::app::screens::utils::centered_rect;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Modifier, StatefulWidget, Style, Widget};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState};

/// Render the action menu overlay
pub fn render(
    area: Rect,
    buf: &mut Buffer,
    app: &App,
    _track_index: usize,
    selected_action: usize,
) {
    // Create a centered popup
    let popup_area = centered_rect(50, 40, area);

    // Clear the area
    Clear.render(popup_area, buf);

    let actions = get_available_actions(app);

    let items: Vec<ListItem> = actions
        .iter()
        .map(|action| ListItem::new(action.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Track Actions")
                .title_bottom(
                    Line::from(" ↑/↓: Navigate | Enter: Select | Esc: Cancel ").centered(),
                ),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    let mut state = ListState::default();
    state.select(Some(selected_action));

    StatefulWidget::render(list, popup_area, buf, &mut state);
}
