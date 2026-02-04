use crate::app::app::App;
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
                        let selected = app.ui.main.selected_track;
                        app.audio.play_track_by_index(selected);
                        return;
                    }
                    Keymap::Pause => {
                        app.audio.toggle_pause();
                        return;
                    }
                    Keymap::NextTrack => {
                        // Only handle if main content is focused
                        if app.ui.is_main_focused() {
                            app.ui.main.select_next(app.audio.tracks.len());
                        }
                        return;
                    }
                    Keymap::PreviousTrack => {
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
                    // Global actions are handled in app.rs
                    _ => {}
                }
            }
            Focus::Sidebar => {
                match action {
                    Keymap::SelectPlaylist => {
                        let playlist_name = app.audio.playlists()
                            .get(app.ui.main.selected_playlist)
                            .map(|p| p.name.clone());

                        if let Some(name) = playlist_name {
                            app.audio.switch_to_playlist(&name);
                        }
                        return;
                    }
                    Keymap::CreatePlaylist => {
                        let playlist_name = format!("New Playlist {}", app.audio.playlists().len() + 1);
                        app.audio.create_playlist(playlist_name);
                        return;
                    }
                    Keymap::DeletePlaylist => {
                        let playlist_name = app.audio.playlists()
                            .get(app.ui.main.selected_playlist)
                            .map(|p| p.name.clone());
                        if let Some(name) = playlist_name {
                            app.audio.delete_playlist(&name);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Fallback hardcoded keys - route based on focus
    match app.ui.focus {
        Focus::MainContent => {
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
        Focus::Sidebar => {
            match key_event.code {
                KeyCode::Down | KeyCode::Char('j') => {
                    app.ui.main.select_next_playlist(app.audio.playlists().len());
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.ui.main.select_previous_playlist();
                }
                KeyCode::Enter => {
                    let playlist_name = app.audio.playlists()
                        .get(app.ui.main.selected_playlist)
                        .map(|p| p.name.clone());

                    if let Some(name) = playlist_name {
                        app.audio.switch_to_playlist(&name);
                    }
                }
                _ => {}
            }
        }
    }
}