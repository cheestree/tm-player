use crate::app::app::App;
use crate::settings::settings::Keymap;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events specific to the main screen
pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    // Check if the pressed key is bound to any action
    if let Some(action) = app.settings.get_action(&key_event.code) {
        match action {
            Keymap::Play => {
                let selected = app.ui.main.selected_track();
                if let Some(track) = app.audio.tracks.get(selected).cloned() {
                    app.audio.play_track(&track);
                }
                return;
            }
            Keymap::Pause => {
                app.audio.toggle_pause();
                return;
            }
            Keymap::NextTrack => {
                app.ui.main.select_next(app.audio.tracks.len());
                return;
            }
            Keymap::PreviousTrack => {
                app.ui.main.select_previous();
                return;
            }
            Keymap::VolumeUp => {
                // TODO: Implement volume control
                return;
            }
            Keymap::VolumeDown => {
                // TODO: Implement volume control
                return;
            }
            Keymap::Rescan => {
                app.rescan_tracks();
                return;
            }
            // Global actions are handled in app.rs
            _ => {}
        }
    }

    // Fallback hardcoded keys
    match key_event.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.ui.main.select_next(app.audio.tracks.len());
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.ui.main.select_previous();
        }
        _ => {}
    }
}