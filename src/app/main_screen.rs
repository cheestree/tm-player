use crate::app::app::App;
use ratatui::prelude::{Color, Modifier, Style, Widget};
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Borders}};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};

pub fn render(area: Rect, buf: &mut Buffer, app: &App) {
    // Sidebar rendering
    if app.ui.side_bar {
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
    // Main area rendering
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Main Screen");
    let main_area = if app.ui.side_bar {
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

fn render_main_content(area: Rect, buf: &mut Buffer, app: &App) {
    let list_items: Vec<ListItem> = app.audio.tracks.iter().map(|track| {
        ListItem::new(format!("{} - {}", track.artist, track.title))
    }).collect();
    let mut state = ListState::default();
    state.select(Some(app.audio.selected_track));
    let list = List::new(list_items)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    Widget::render(&list, area, buf);
}