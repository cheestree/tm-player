use crate::app::app::App;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events specific to the main screen
pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.ui.main.select_next(app.audio.tracks.len());
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.ui.main.select_previous();
        }
        KeyCode::Enter => {
            let selected = app.ui.main.selected_track();
            if let Some(track) = app.audio.tracks.get(selected).cloned() {
                app.audio.play_track(&track);
            }
        }
        KeyCode::Char(' ') => {
            app.audio.toggle_pause();
        }
        _ => {}
    }
}