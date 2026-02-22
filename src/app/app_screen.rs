/// Represents the different screens in the application.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum AppScreen {
    #[default]
    Main,
    Help,
}

/// Menu actions available for tracks
#[derive(Debug, Clone)]
pub enum TrackAction {
    AddToPlaylist,
    RemoveFromPlaylist,
    MoveUp,
    MoveDown,
}

impl TrackAction {
    pub fn as_str(&self) -> &str {
        match self {
            TrackAction::AddToPlaylist => "Add to Playlist",
            TrackAction::RemoveFromPlaylist => "Remove from Playlist",
            TrackAction::MoveUp => "Move Up",
            TrackAction::MoveDown => "Move Down",
        }
    }
}

/// Represents the different overlays in the application.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Overlay {
    Settings,
    DeletePlaylist {
        title: String,
        description: Option<String>
    },
    ActionMenu {
        track_index: usize,
        selected_action: usize,
    },
    /// Playlist selection for adding a track
    AddToPlaylist {
        track_id: u64,
        selected_playlist: usize,
    },
    /// Rename playlist with text input
    RenamePlaylist {
        playlist_index: usize,
        current_name: String,
    },
}
