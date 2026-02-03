use crate::app::app::App;
use crate::settings::settings::Keymap;
use crossterm::event::KeyCode;
use ratatui::prelude::{Color, Modifier, StatefulWidget, Style, Widget};
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Borders, Row, Table, TableState}};
use ratatui::layout::Constraint;
use ratatui::text::{Line, Span};

pub fn render(area: Rect, buf: &mut Buffer, app: &App) {
    // Sidebar rendering
    if app.ui.side_bar() {
        let sidebar_block = Block::default()
            .borders(Borders::ALL)
            .title("Sidebar");
        let sidebar_area = Rect {
            x: area.x,
            y: area.y,
            width: 20,
            height: area.height,
        };
        sidebar_block.render(sidebar_area, buf);
    }

    // Build bottom keybindings display
    let keybindings = build_keybinding_line(app);

    // Main area rendering with keybindings on bottom border
    let block = Block::default()
        .borders(Borders::ALL)
        .title("My Tracks")
        .title_bottom(keybindings);

    let main_area = if app.ui.side_bar() {
        Rect {
            x: area.x + 20,
            y: area.y,
            width: area.width - 20,
            height: area.height,
        }
    } else {
        area
    };
    block.render(main_area, buf);

    // Render main content (track list)
    let inner_area = Rect {
        x: main_area.x + 1,
        y: main_area.y + 1,
        width: main_area.width - 2,
        height: main_area.height - 2,
    };
    render_main_content(inner_area, buf, app);
}

fn build_keybinding_line(app: &App) -> Line<'static> {
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

    let mut first = true;
    for action in action_order.iter() {
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
            spans.push(Span::styled(key_str, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
        }
    }

    spans.push(Span::raw(" "));

    Line::from(spans).centered()
}

fn render_main_content(area: Rect, buf: &mut Buffer, app: &App) {
    let rows: Vec<Row> = app.audio.tracks.iter().map(|track| {
        let total_secs = track.duration.as_secs();
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;
        Row::new(vec![
            track.artist.clone(),
            track.title.clone(),
            track.album.clone(),
            format!("{:02}:{:02}", minutes, seconds),
        ])
    }).collect();

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
        Row::new(vec!["Artist", "Title", "Album", "Duration"])
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    )
    .row_highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD));

    let mut state = TableState::default();
    state.select(Some(app.ui.main.selected_track()));

    // Render the table
    StatefulWidget::render(table, area, buf, &mut state);
}