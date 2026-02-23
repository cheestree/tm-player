use crate::app::app::App;
use crate::app::app_screen::{Overlay, TrackAction};
use crate::app::screens::main::utils::compute_sorted_indices;
use crate::app::screens::overlay::actionmenu::common::get_available_actions;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events for the action menu
pub fn handle_key_event(
    app: &mut App,
    key_event: KeyEvent,
    track_index: usize,
    selected_action: usize,
) {
    let actions = get_available_actions(app);

    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.ui.overlay = None;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let new_selection = if selected_action == 0 {
                actions.len().saturating_sub(1)
            } else {
                selected_action - 1
            };
            app.ui.overlay = Some(Overlay::ActionMenu {
                track_index,
                selected_action: new_selection,
            });
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let new_selection = (selected_action + 1) % actions.len();
            app.ui.overlay = Some(Overlay::ActionMenu {
                track_index,
                selected_action: new_selection,
            });
        }
        KeyCode::Enter => {
            if let Some(action) = actions.get(selected_action) {
                execute_action(app, action, track_index);
            }
        }
        _ => {}
    }
}

/// Execute the selected action
fn execute_action(app: &mut App, action: &TrackAction, visual_index: usize) {
    let playlist_idx = app.ui.main.selected_playlist;

    // Get the track ID from the visual position
    let tracks = app.audio.get_playlist_tracks(playlist_idx);
    let sorted_indices =
        compute_sorted_indices(&tracks, &app.ui.main.sorted_by, app.ui.main.sort_ascending);

    // Get the actual index in the tracks vector (sorted view)
    let sorted_track_index = sorted_indices
        .get(visual_index)
        .copied()
        .unwrap_or(visual_index);

    // Get the track at that position
    let track = match tracks.get(sorted_track_index) {
        Some(t) => t,
        None => return, // Invalid index
    };

    let track_id = track.get_id();

    // Now find the position of this track ID in the actual playlist's track_ids array
    let actual_playlist_position = app
        .audio
        .playlists
        .get(playlist_idx)
        .and_then(|playlist| playlist.track_ids.iter().position(|&id| id == track_id));

    match action {
        TrackAction::AddToPlaylist => {
            app.ui.overlay = Some(Overlay::AddToPlaylist {
                track_id,
                selected_playlist: 0,
            });
        }
        TrackAction::RemoveFromPlaylist => {
            if let Some(playlist) = app.audio.playlists.get_mut(playlist_idx) {
                playlist.track_ids.retain(|&id| id != track_id);
            }
            app.save_playlists_now(); // Save immediately

            // Clamp selected_track if it's now out of bounds
            let new_len = app.audio.get_playlist_tracks(playlist_idx).len();
            if app.ui.main.selected_track >= new_len && new_len > 0 {
                app.ui.main.selected_track = new_len - 1;
            }

            app.ui.overlay = None;
        }
        TrackAction::MoveUp => {
            // Only called when not sorted, so visual_index == actual position
            if let Some(position) = actual_playlist_position {
                if app.audio.move_track_up_in_playlist(playlist_idx, position) {
                    // Move selection up to follow the track
                    if app.ui.main.selected_track > 0 {
                        app.ui.main.selected_track -= 1;
                    }
                }
                app.save_playlists_now();
            }
            app.ui.overlay = None;
        }
        TrackAction::MoveDown => {
            // Only called when not sorted, so visual_index == actual position
            if let Some(position) = actual_playlist_position {
                let track_count = app.audio.get_playlist_tracks(playlist_idx).len();
                if app
                    .audio
                    .move_track_down_in_playlist(playlist_idx, position)
                {
                    // Move selection down to follow the track
                    if app.ui.main.selected_track + 1 < track_count {
                        app.ui.main.selected_track += 1;
                    }
                }
                app.save_playlists_now();
            }
            app.ui.overlay = None;
        }
    }
}
