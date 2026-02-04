use crate::track::track::Track;
use rodio::{OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

/// A named playlist containing track IDs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    /// Track IDs (using metadata-based hashes)
    pub track_ids: Vec<u64>,
}

/// Playlist implementation
impl Playlist {
    pub fn new(name: String, track_ids: Vec<u64>) -> Self {
        Self { name, track_ids }
    }
}

/// Serializable playlist data (saved to JSON)
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistData {
    pub playlists: Vec<Playlist>,
    pub current_playlist: String,
}

/// The audio state of the application
pub struct AudioState {
    /// All available tracks (the library)
    pub tracks: Vec<Track>,
    /// Map of track ID -> index for quick lookup
    track_id_map: HashMap<u64, usize>,
    /// Named playlists
    pub playlists: Vec<Playlist>,
    /// Currently active playlist index
    current_playlist_index: usize,
    /// Current position in the active playlist
    current_track_position: usize,
    _stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
}

impl AudioState {
    pub fn new(tracks: Vec<Track>, stream: OutputStream, sink: Arc<Mutex<Sink>>) -> Self {
        // Build track ID -> index map
        let track_id_map: HashMap<u64, usize> = tracks
            .iter()
            .enumerate()
            .map(|(idx, track)| (track.get_id(), idx))
            .collect();

        // Create default "All Tracks" playlist
        let all_tracks_ids: Vec<u64> = tracks.iter().map(|t| t.get_id()).collect();
        let default_playlist = Playlist::new("All Tracks".to_string(), all_tracks_ids);

        Self {
            tracks,
            track_id_map,
            playlists: vec![default_playlist],
            current_playlist_index: 0,
            current_track_position: 0,
            _stream: stream,
            sink,
        }
    }

    /// Load playlists from saved data
    pub fn with_playlists(
        tracks: Vec<Track>,
        stream: OutputStream,
        sink: Arc<Mutex<Sink>>,
        playlist_data: PlaylistData,
    ) -> Self {
        let track_id_map: HashMap<u64, usize> = tracks
            .iter()
            .enumerate()
            .map(|(idx, track)| (track.get_id(), idx))
            .collect();

        // Ensure "All Tracks" playlist always exists
        let all_tracks_ids: Vec<u64> = tracks.iter().map(|t| t.get_id()).collect();
        let mut playlists = playlist_data.playlists;

        // Check if "All Tracks" exists, if not add it
        if !playlists.iter().any(|p| p.name == "All Tracks") {
            playlists.insert(0, Playlist::new("All Tracks".to_string(), all_tracks_ids));
        } else {
            // Update "All Tracks" to match current library
            if let Some(all_tracks_pl) = playlists.iter_mut().find(|p| p.name == "All Tracks") {
                all_tracks_pl.track_ids = all_tracks_ids;
            }
        }

        // Find current playlist index
        let current_playlist_index = playlists
            .iter()
            .position(|p| p.name == playlist_data.current_playlist)
            .unwrap_or(0);

        Self {
            tracks,
            track_id_map,
            playlists,
            current_playlist_index,
            current_track_position: 0,
            _stream: stream,
            sink,
        }
    }

    /// Save playlists to JSON format
    pub fn get_playlist_data(&self) -> PlaylistData {
        PlaylistData {
            playlists: self.playlists.clone(),
            current_playlist: self
                .playlists
                .get(self.current_playlist_index)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "All Tracks".to_string()),
        }
    }

    /// Save playlists to file
    pub fn save_playlists(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.get_playlist_data();
        let json = serde_json::to_string_pretty(&data)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load playlists from file
    pub fn load_playlists(path: &str) -> Result<PlaylistData, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let data: PlaylistData = serde_json::from_str(&contents)?;
        Ok(data)
    }

    /// Play a track by its index in the tracks library
    pub fn play_track_by_index(&mut self, track_index: usize) {
        if let Some(track) = self.tracks.get(track_index) {
            let path = &track.path;
            let sink = self.sink.lock().unwrap();
            let file = std::fs::File::open(path).expect("open audio file");
            let source = rodio::Decoder::new(io::BufReader::new(file)).expect("decode audio file");
            sink.stop();
            sink.append(source);
            sink.play();
        }
    }

    /// Toggle pause/play state
    pub fn toggle_pause(&mut self) {
        let sink = self.sink.lock().unwrap();
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }

    /// Get the currently playing track
    pub fn get_current_track(&self) -> Option<&Track> {
        let playlist = self.playlists.get(self.current_playlist_index)?;
        let track_id = playlist.track_ids.get(self.current_track_position)?;
        let track_index = self.track_id_map.get(track_id)?;
        self.tracks.get(*track_index)
    }

    /// Set current track by library index (finds it in current playlist)
    pub fn set_current_track(&mut self, track_index: usize) {
        if let Some(track) = self.tracks.get(track_index) {
            let track_id = track.get_id();
            if let Some(playlist) = self.playlists.get(self.current_playlist_index) {
                if let Some(pos) = playlist.track_ids.iter().position(|&id| id == track_id) {
                    self.current_track_position = pos;
                }
            }
        }
    }

    /// Move to next track in current playlist
    pub fn next_track(&mut self) -> Option<&Track> {
        let playlist = self.playlists.get(self.current_playlist_index)?;
        if playlist.track_ids.is_empty() {
            return None;
        }
        self.current_track_position = (self.current_track_position + 1) % playlist.track_ids.len();
        self.get_current_track()
    }

    /// Move to previous track in current playlist
    pub fn previous_track(&mut self) -> Option<&Track> {
        let playlist = self.playlists.get(self.current_playlist_index)?;
        if playlist.track_ids.is_empty() {
            return None;
        }
        if self.current_track_position == 0 {
            self.current_track_position = playlist.track_ids.len() - 1;
        } else {
            self.current_track_position -= 1;
        }
        self.get_current_track()
    }

    /// Get current playlist
    pub fn current_playlist(&self) -> Option<&Playlist> {
        self.playlists.get(self.current_playlist_index)
    }

    /// Get all playlists
    pub fn playlists(&self) -> &[Playlist] {
        &self.playlists
    }

    /// Switch to a different playlist by name
    pub fn switch_to_playlist(&mut self, name: &str) {
        if let Some(index) = self.playlists.iter().position(|p| p.name == name) {
            self.current_playlist_index = index;
            self.current_track_position = 0;
        }
    }

    /// Create a new playlist
    pub fn create_playlist(&mut self, name: String) {
        let playlist = Playlist::new(name, Vec::new());
        self.playlists.push(playlist);
    }

    /// Add track to a playlist by name
    pub fn add_track_to_playlist(&mut self, playlist_name: &str, track_index: usize) {
        if let Some(track) = self.tracks.get(track_index) {
            let track_id = track.get_id();
            if let Some(playlist) = self.playlists.iter_mut().find(|p| p.name == playlist_name) {
                if !playlist.track_ids.contains(&track_id) {
                    playlist.track_ids.push(track_id);
                }
            }
        }
    }

    /// Remove track from a playlist
    pub fn remove_track_from_playlist(&mut self, playlist_name: &str, track_id: u64) {
        if let Some(playlist) = self.playlists.iter_mut().find(|p| p.name == playlist_name) {
            playlist.track_ids.retain(|&id| id != track_id);
        }
    }

    /// Delete a playlist (except "All Tracks")
    pub fn delete_playlist(&mut self, name: &str) -> bool {
        if name == "All Tracks" {
            return false; // Can't delete the default playlist
        }

        if let Some(index) = self.playlists.iter().position(|p| p.name == name) {
            self.playlists.remove(index);
            // Adjust current index if needed
            if self.current_playlist_index >= self.playlists.len() {
                self.current_playlist_index = 0;
            }
            true
        } else {
            false
        }
    }

    /// Get track indices for current playlist (for display purposes)
    pub fn get_current_playlist_tracks(&self) -> Vec<usize> {
        if let Some(playlist) = self.playlists.get(self.current_playlist_index) {
            playlist
                .track_ids
                .iter()
                .filter_map(|id| self.track_id_map.get(id).copied())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get tracks from a list of track IDs
    pub fn get_tracks_by_ids(&self, track_ids: &[u64]) -> Vec<&Track> {
        track_ids
            .iter()
            .filter_map(|id| {
                self.track_id_map
                    .get(id)
                    .and_then(|&idx| self.tracks.get(idx))
            })
            .collect()
    }

    /// Get tracks for a specific playlist by index
    pub fn get_playlist_tracks(&self, playlist_index: usize) -> Vec<&Track> {
        if let Some(playlist) = self.playlists.get(playlist_index) {
            self.get_tracks_by_ids(&playlist.track_ids)
        } else {
            Vec::new()
        }
    }
}
