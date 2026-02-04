use crate::app::app::App;
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
    let sidebar_width = if app.ui.side_bar { 50 } else { 0 };

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
    let mut spans = Vec::new();
    spans.push(Span::raw(" "));

    let action_order = [
        Keymap::Play,
        Keymap::Pause,
        Keymap::NextTrack,
        Keymap::PreviousTrack,
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
        Keymap::SelectPlaylist,
        Keymap::CreatePlaylist,
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
    let playlists = app.audio.playlists();

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

    let rows: Vec<Row> = tracks
        .iter()
        .map(|track| {
            let total_secs = track.duration.as_secs();
            let minutes = total_secs / 60;
            let seconds = total_secs % 60;
            // Use Cell to avoid cloning - Cell accepts &str
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

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(15), // Artist
            Constraint::Percentage(35), // Title
            Constraint::Percentage(40), // Album
            Constraint::Percentage(10), // Duration
        ],
    )
    .header(
        Row::new(vec!["Artist", "Title", "Album", "Duration"]).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    )
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
