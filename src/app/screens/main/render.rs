use crate::app::app::App;
use crate::app::screens::main::state::TrackSort;
use crate::app::screens::main::utils::compute_sorted_indices;
use crate::settings::settings::Keymap;
use crossterm::event::KeyCode;
use ratatui::layout::Constraint;
use ratatui::prelude::{Color, Modifier, StatefulWidget, Style};
use ratatui::text::{Line, Span};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Cell, List, ListItem, ListState, Row, Table, TableState},
};

pub fn render(area: Rect, buf: &mut Buffer, app: &App) {
    let sidebar_width = if app.ui.side_bar { 30 } else { 0 };

    // Sidebar rendering
    if app.ui.side_bar {
        let sidebar_area = Rect {
            x: area.x,
            y: area.y,
            width: sidebar_width,
            height: area.height,
        };
        render_sidebar(sidebar_area, buf, app);
    }

    // Main area rendering
    let main_area = if app.ui.side_bar {
        Rect {
            x: area.x + sidebar_width,
            y: area.y,
            width: area.width - sidebar_width,
            height: area.height,
        }
    } else {
        area
    };

    render_main_content(main_area, buf, app);
}

fn build_main_keybinding_line(app: &App) -> Line<'static> {
    if !app.ui.is_main_focused() {
        return Line::from("");
    }
    let mut spans = vec![Span::raw(" ")];

    let action_order = [
        Keymap::Play,
        Keymap::Pause,
        Keymap::OpenActionMenu,
        Keymap::SelectNextTrack,
        Keymap::SelectPreviousTrack,
        Keymap::NextTrack,
        Keymap::PreviousTrack,
        Keymap::ToggleShuffle,
        Keymap::VolumeUp,
        Keymap::VolumeDown,
    ];

    spans = build_keybinding_line(spans, action_order.to_vec(), app);

    spans.push(Span::raw(" "));

    Line::from(spans).centered()
}

fn build_sidebar_keybinding_line(app: &App) -> Line<'static> {
    if !app.ui.is_sidebar_focused() {
        return Line::from("");
    }
    let mut spans = vec![Span::raw(" ")];

    let action_order = [
        Keymap::CreatePlaylist,
        Keymap::RenamePlaylist,
        Keymap::DeletePlaylist,
    ];

    spans = build_keybinding_line(spans, action_order.to_vec(), app);

    spans.push(Span::raw(" "));

    Line::from(spans).centered()
}

fn build_keybinding_line<'a>(
    mut spans: Vec<Span<'a>>,
    actions: Vec<Keymap>,
    app: &App,
) -> Vec<Span<'a>> {
    let mut first = true;
    for action in actions.iter() {
        // Find the key bound to this action
        if let Some(keycode) = app.settings.find_key_for_action(action) {
            if !first {
                spans.push(Span::raw(" │ "));
            }
            first = false;

            // Action name in default color
            spans.push(Span::raw(action.as_str()));
            spans.push(Span::raw(":"));

            // Key in highlighted color
            let key_str = match keycode {
                KeyCode::Char(' ') => " Space".to_string(),
                KeyCode::Char(c) => format!(" '{}'", c),
                _ => format!(" {:?}", keycode),
            };
            spans.push(Span::styled(
                key_str,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        }
    }
    spans
}

fn render_sidebar(area: Rect, buf: &mut Buffer, app: &App) {
    let playlists = &app.audio.playlists;

    let items: Vec<ListItem> = playlists
        .iter()
        .map(|playlist| ListItem::new(&playlist.name[..]))
        .collect();

    let keybindings = build_sidebar_keybinding_line(app);

    let title_line = if app.ui.is_sidebar_focused() {
        Line::from("Playlists").style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Line::from("Playlists")
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title_line)
                .title_bottom(keybindings),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    let mut state = ListState::default();
    state.select(Some(app.ui.main.selected_playlist));

    StatefulWidget::render(list, area, buf, &mut state);
}

fn render_main_content(area: Rect, buf: &mut Buffer, app: &App) {
    let tracks = app.audio.get_playlist_tracks(app.ui.main.selected_playlist);

    // Compute sorted indices
    let sorted_indices =
        compute_sorted_indices(&tracks, &app.ui.main.sorted_by, app.ui.main.sort_ascending);

    let rows: Vec<Row> = sorted_indices
        .iter()
        .filter_map(|&idx| tracks.get(idx))
        .map(|track| {
            let total_secs = track.duration.as_secs();
            let minutes = total_secs / 60;
            let seconds = total_secs % 60;
            Row::new(vec![
                Cell::from(&track.artist[..]),
                Cell::from(&track.title[..]),
                Cell::from(&track.album[..]),
                Cell::from(format!("{:02}:{:02}", minutes, seconds)),
            ])
        })
        .collect();

    let keybindings = build_main_keybinding_line(app);

    let title_line = if app.ui.is_main_focused() {
        Line::from("My Tracks").style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Line::from("My Tracks")
    };

    let mut header_cells = vec![
        "Artist".to_string(),
        "Title".to_string(),
        "Album".to_string(),
        "Duration".to_string(),
    ];
    if let Some(sorted_column) = match app.ui.main.sorted_by {
        Some(TrackSort::Artist) => Some(0),
        Some(TrackSort::Title) => Some(1),
        Some(TrackSort::Album) => Some(2),
        Some(TrackSort::Duration) => Some(3),
        _ => None,
    } {
        if sorted_column < header_cells.len() {
            let arrow = if app.ui.main.sort_ascending {
                " ▲"
            } else {
                " ▼"
            };
            header_cells[sorted_column].push_str(arrow);
        }
    }

    let header = Row::new(header_cells).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(15), // Artist
            Constraint::Percentage(35), // Title
            Constraint::Percentage(40), // Album
            Constraint::Percentage(10), // Duration
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title_line)
            .title_bottom(keybindings),
    )
    .row_highlight_style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let mut state = TableState::default();
    state.select(Some(app.ui.main.selected_track));

    StatefulWidget::render(table, area, buf, &mut state);
}
