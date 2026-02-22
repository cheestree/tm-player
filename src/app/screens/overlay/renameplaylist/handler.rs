use crate::app::app::App;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events for rename playlist
pub fn handle_key_event(
    app: &mut App,
    key_event: KeyEvent,
    playlist_index: usize,
    current_name: &str,
) {
    match key_event.code {
        KeyCode::Esc => {
            app.ui.overlay = None;
        }
        KeyCode::Enter => {
            // Save the new name
            if !current_name.is_empty() && current_name != "All Tracks" {
                if let Some(playlist) = app.audio.playlists.get_mut(playlist_index) {
                    playlist.name = current_name.to_string();
                }
                app.save_playlists_now(); // Save immediately
            }
            app.ui.overlay = None;
        }
        KeyCode::Backspace => {
            // Remove last character
            let mut new_name = current_name.to_string();
            new_name.pop();
            app.ui.overlay = Some(crate::app::app_screen::Overlay::RenamePlaylist {
                playlist_index,
                current_name: new_name,
            });
        }
        KeyCode::Char(c) => {
            // Add character (limit to reasonable length)
            if current_name.len() < 50 {
                let mut new_name = current_name.to_string();
                new_name.push(c);
                app.ui.overlay = Some(crate::app::app_screen::Overlay::RenamePlaylist {
                    playlist_index,
                    current_name: new_name,
                });
            }
        }
        _ => {}
    }
}
