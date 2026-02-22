use crate::app::app::App;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events for the AreYouSure overlay
pub fn handle_key_event(
    app: &mut App,
    key_event: KeyEvent,
    _title: &str,
    _description: &Option<String>
) {
    match key_event.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
            let playlist_name = app.audio.playlists.get(app.ui.main.selected_playlist).map(|p| p.name.clone()).unwrap_or_default();
            app.audio.delete_playlist(&playlist_name);
            app.save_playlists_now(); // Save immediately
            app.ui.overlay = None;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.ui.overlay = None;
        }
        _ => {}
    }
}

