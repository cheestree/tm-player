pub mod actionmenu;
pub mod addtoplaylist;
pub mod deleteplaylist;
pub mod renameplaylist;
pub mod settings;

use crate::app::app::App;
use crate::app::app_screen::Overlay;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

/// Central overlay rendering function that dispatches to the appropriate overlay
pub fn render(area: Rect, buf: &mut Buffer, app: &App, overlay: &Overlay) {
    match overlay {
        Overlay::Settings => settings::render(area, buf, app),
        Overlay::DeletePlaylist { title, description } => {
            deleteplaylist::render(area, buf, app, title, description);
        }
        Overlay::ActionMenu {
            track_index,
            selected_action,
        } => {
            actionmenu::render(area, buf, app, *track_index, *selected_action);
        }
        Overlay::AddToPlaylist {
            track_id,
            selected_playlist,
        } => {
            addtoplaylist::render(area, buf, app, *track_id, *selected_playlist);
        }
        Overlay::RenamePlaylist {
            playlist_index,
            current_name,
        } => {
            renameplaylist::render(area, buf, app, *playlist_index, current_name);
        }
    }
}

/// Central overlay key event handler that dispatches to the appropriate overlay
pub fn handle_key_event(app: &mut App, key_event: crossterm::event::KeyEvent, overlay: &Overlay) {
    match overlay {
        Overlay::Settings => settings::handle_key_event(app, key_event),
        Overlay::DeletePlaylist { title, description } => {
            deleteplaylist::handle_key_event(app, key_event, title, description)
        }
        Overlay::ActionMenu {
            track_index,
            selected_action,
        } => actionmenu::handle_key_event(app, key_event, *track_index, *selected_action),
        Overlay::AddToPlaylist {
            track_id,
            selected_playlist,
        } => addtoplaylist::handle_key_event(app, key_event, *track_id, *selected_playlist),
        Overlay::RenamePlaylist {
            playlist_index,
            current_name,
        } => renameplaylist::handle_key_event(app, key_event, *playlist_index, current_name),
    }
}
