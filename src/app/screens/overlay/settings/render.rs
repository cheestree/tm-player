use crate::app::app::App;
use ratatui::prelude::{Color, Modifier, Style, Widget};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders},
};

pub fn render(area: Rect, buf: &mut Buffer, app: &App) {
    let popup_width = area.width / 2;
    let popup_height = area.height / 3;
    let popup_x = area.x + (area.width - popup_width) / 2;
    let popup_y = area.y + (area.height - popup_height) / 2;
    let popup_area = Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    };

    let block = Block::default().borders(Borders::ALL).title("Settings");
    block.render(popup_area, buf);

    let inner_area = Rect {
        x: popup_area.x + 1,
        y: popup_area.y + 1,
        width: popup_area.width - 2,
        height: popup_area.height - 2,
    };
    render_menu(
        inner_area,
        buf,
        &["Option 1", "Option 2", "Option 3"],
        app.ui.settings.selected_index,
    );
}

fn render_menu(area: Rect, buf: &mut Buffer, items: &[&str], selected: usize) {
    let list_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(*i)).collect();
    let mut state = ListState::default();
    state.select(Some(selected));
    let list = List::new(list_items).highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );
    Widget::render(&list, area, buf);
}
