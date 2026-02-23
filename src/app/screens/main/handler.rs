use crate::app::app::App;
use crate::app::app_screen::Overlay;
use crate::app::screens::main::utils::compute_sorted_indices;
use crate::app::state::Focus;
use crate::settings::settings::Keymap;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events specific to the main screen
pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    // Check if the pressed key is bound to any action
    if let Some(action) = app.settings.get_action(&key_event.code) {
        match app.ui.focus {
            Focus::MainContent => {
                match action {
                    Keymap::Play => {
                        // Get the actual track index from the sorted view
                        let tracks = app.audio.get_playlist_tracks(app.ui.main.selected_playlist);
                        let sorted_indices = compute_sorted_indices(
                            &tracks,
                            &app.ui.main.sorted_by,
                            app.ui.main.sort_ascending,
                        );

                        if let Some(&actual_idx) = sorted_indices.get(app.ui.main.selected_track)
                            && let Some(track) = tracks.get(actual_idx)
                        {
                            // Find the track's library index
                            if let Some(lib_idx) = app
                                .audio
                                .tracks
                                .iter()
                                .position(|t| t.get_id() == track.get_id())
                            {
                                // Switch to the selected playlist before playing
                                app.audio
                                    .switch_to_playlist_by_index(app.ui.main.selected_playlist);
                                app.audio.play_track_by_index(lib_idx);
                            }
                        }
                        return;
                    }
                    Keymap::Pause => {
                        app.audio.toggle_pause();
                        return;
                    }
                    Keymap::NextTrack => {
                        app.audio
                            .play_next_track_in_playlist(app.ui.main.selected_playlist);
                        return;
                    }
                    Keymap::PreviousTrack => {
                        app.audio
                            .play_previous_track_in_playlist(app.ui.main.selected_playlist);
                        return;
                    }
                    Keymap::SelectNextTrack => {
                        // Only handle if main content is focused
                        if app.ui.is_main_focused() {
                            app.ui.main.select_next(app.audio.tracks.len());
                        }
                        return;
                    }
                    Keymap::SelectPreviousTrack => {
                        // Only handle if main content is focused
                        if app.ui.is_main_focused() {
                            app.ui.main.select_previous();
                        }
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
                    Keymap::Sort => {
                        app.ui.main.cycle_sort();
                        return;
                    }
                    Keymap::ToggleShuffle => {
                        app.audio.toggle_shuffle();
                        return;
                    }
                    Keymap::OpenActionMenu => {
                        app.ui.overlay = Some(Overlay::ActionMenu {
                            track_index: app.ui.main.selected_track,
                            selected_action: 0,
                        });
                        return;
                    }
                    // Global actions are handled in app.rs
                    _ => {}
                }
            }
            Focus::Sidebar => match action {
                Keymap::SelectPlaylist => {
                    let playlist_name = app
                        .audio
                        .playlists
                        .get(app.ui.main.selected_playlist)
                        .map(|p| p.name.clone());

                    if let Some(name) = playlist_name {
                        app.audio.switch_to_playlist(&name);
                        // Reset sorting to None when switching to a custom playlist
                        if app.ui.main.selected_playlist > 0 {
                            app.ui.main.sorted_by = None;
                        }
                    }
                    return;
                }
                Keymap::CreatePlaylist => {
                    let playlist_name = format!("New Playlist {}", app.audio.playlists.len() + 1);
                    app.audio.create_playlist(playlist_name);
                    app.save_playlists_now(); // Save immediately
                    // Reset sorting to None so tracks can be moved
                    app.ui.main.sorted_by = None;
                    return;
                }
                Keymap::DeletePlaylist => {
                    let playlist_name = app
                        .audio
                        .playlists
                        .get(app.ui.main.selected_playlist)
                        .map(|p| p.name.clone());

                    if let Some(name) = playlist_name {
                        app.ui.overlay = Some(Overlay::DeletePlaylist {
                            title: "Delete Playlist".to_string(),
                            description: Some(format!(
                                "Are you sure you want to delete the playlist '{}'?",
                                name
                            )),
                        });
                    }
                    return;
                }
                Keymap::RenamePlaylist => {
                    let playlist_index = app.ui.main.selected_playlist;
                    // Don't allow renaming "All Tracks"
                    if playlist_index > 0
                        && let Some(playlist) = app.audio.playlists.get(playlist_index)
                    {
                        app.ui.overlay = Some(Overlay::RenamePlaylist {
                            playlist_index,
                            current_name: playlist.name.clone(),
                        });
                    }
                    return;
                }
                _ => {}
            },
        }
    }

    // Fallback hardcoded keys - route based on focus
    match app.ui.focus {
        Focus::MainContent => match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => {
                app.ui.main.select_next(app.audio.tracks.len());
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.ui.main.select_previous();
            }
            _ => {}
        },
        Focus::Sidebar => match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => {
                app.ui.main.select_next_playlist(app.audio.playlists.len());
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.ui.main.select_previous_playlist();
            }
            KeyCode::Enter => {
                let playlist_name = app
                    .audio
                    .playlists
                    .get(app.ui.main.selected_playlist)
                    .map(|p| p.name.clone());

                if let Some(name) = playlist_name {
                    app.audio.switch_to_playlist(&name);
                }
            }
            _ => {}
        },
    }
}
