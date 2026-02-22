use crate::app::app::App;
use crate::app::app_screen::Overlay;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events for the "Add to Playlist" overlay
pub fn handle_key_event(
    app: &mut App,
    key_event: KeyEvent,
    track_id: u64,
    selected_playlist: usize,
) {
    // Get playlists except "All Tracks"
    let playlists: Vec<_> = app
        .audio
        .playlists
        .iter()
        .enumerate()
        .filter(|(_, p)| p.name != "All Tracks")
        .map(|(idx, _)| idx)
        .collect();

    let max_idx = playlists.len().saturating_sub(1);
    let current_filtered = if selected_playlist > 0 {
        selected_playlist.saturating_sub(1).min(max_idx)
    } else {
        0
    };

    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.ui.overlay = None;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let new_filtered = if current_filtered == 0 {
                max_idx
            } else {
                current_filtered - 1
            };
            let new_playlist = playlists.get(new_filtered).copied().unwrap_or(1);
            app.ui.overlay = Some(Overlay::AddToPlaylist {
                track_id,
                selected_playlist: new_playlist,
            });
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let new_filtered = if current_filtered >= max_idx {
                0
            } else {
                current_filtered + 1
            };
            let new_playlist = playlists.get(new_filtered).copied().unwrap_or(1);
            app.ui.overlay = Some(Overlay::AddToPlaylist {
                track_id,
                selected_playlist: new_playlist,
            });
        }
        KeyCode::Enter => {
            if let Some(&playlist_idx) = playlists.get(current_filtered) {
                app.audio.add_track_id_to_playlist(playlist_idx, track_id);
                app.save_playlists_now(); // Save immediately
            }
            app.ui.overlay = None;
        }
        _ => {}
    }
}
