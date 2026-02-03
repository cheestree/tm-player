use crate::app::app::App;
use crate::settings::settings::Keymap;
use crossterm::event::KeyCode;
use ratatui::prelude::{Color, Modifier, StatefulWidget, Style, Widget};
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Borders}};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState};

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

    let keymap_order = [
        Keymap::Play,
        Keymap::Pause,
        Keymap::NextTrack,
        Keymap::PreviousTrack,
        Keymap::VolumeUp,
        Keymap::VolumeDown,
    ];

    let mut first = true;
    for action in keymap_order.iter() {
        if let Some(keycode) = app.settings.get_keycode(action) {
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
                _ => format!(" {:?}", keycode),
            };
            spans.push(Span::styled(key_str, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
        }
    }

    spans.push(Span::raw(" "));

    Line::from(spans).centered()
}

fn render_main_content(area: Rect, buf: &mut Buffer, app: &App) {
    let list_items: Vec<ListItem> = app.audio.tracks.iter().map(|track| {
        ListItem::new(format!("{} - {}", track.artist, track.title))
    }).collect();

    let mut state = ListState::default();
    state.select(Some(app.ui.main.selected_track()));

    let list = List::new(list_items)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    StatefulWidget::render(&list, area, buf, &mut state);
}
