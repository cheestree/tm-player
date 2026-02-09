pub mod areyousure;
pub mod settings;

use crate::app::app::App;
use crate::app::app_screen::Overlay;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

/// Central overlay rendering function that dispatches to the appropriate overlay
pub fn render(area: Rect, buf: &mut Buffer, app: &App, overlay: &Overlay) {
    match overlay {
        Overlay::Settings => settings::render(area, buf, app),
        Overlay::AreYouSure {
            title,
            description,
            action: _,
        } => {
            areyousure::render(area, buf, app, title, description);
        }
        Overlay::DeletePlaylist => {
            // TODO: Implement DeletePlaylist overlay
        }
    }
}

/// Central overlay key event handler that dispatches to the appropriate overlay
pub fn handle_key_event(app: &mut App, key_event: crossterm::event::KeyEvent, overlay: &Overlay) {
    match overlay {
        Overlay::Settings => settings::handle_key_event(app, key_event),
        Overlay::AreYouSure {
            title,
            description,
            action,
        } => areyousure::handle_key_event(app, key_event, title, description, action),
        Overlay::DeletePlaylist => {
            // TODO: Implement DeletePlaylist handler
        }
    }
}
