use crate::app::app::App;
use crate::app::screens::utils::centered_rect;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, StatefulWidget, Style, Widget};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState};

/// Render the "Add to Playlist" selection overlay
pub fn render(area: Rect, buf: &mut Buffer, app: &App, _track_id: u64, selected_playlist: usize) {
    // Create a centered popup
    let popup_area = centered_rect(60, 50, area);

    // Clear the area
    Clear.render(popup_area, buf);

    // Get all playlists except "All Tracks"
    let playlists: Vec<_> = app
        .audio
        .playlists
        .iter()
        .enumerate()
        .filter(|(_, p)| p.name != "All Tracks")
        .collect();

    let items: Vec<ListItem> = playlists
        .iter()
        .map(|(_, playlist)| {
            let track_count = playlist.track_ids.len();
            ListItem::new(format!("{} ({} tracks)", playlist.name, track_count))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Add to Playlist")
                .title_bottom(Line::from(" ↑/↓: Navigate | Enter: Add | Esc: Cancel ").centered()),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    let mut state = ListState::default();
    // Map selected_playlist to the filtered index
    let filtered_selection = if selected_playlist > 0 {
        selected_playlist
            .saturating_sub(1)
            .min(playlists.len().saturating_sub(1))
    } else {
        0
    };
    state.select(Some(filtered_selection));

    StatefulWidget::render(list, popup_area, buf, &mut state);
}
