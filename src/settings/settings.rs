use std::fmt::{Debug, Formatter};

pub struct Settings {
    pub music_paths: Vec<String>,
    pub volume_level: u8,
    pub shuffle: bool,
}

impl Settings {
    pub fn new(music_paths: Vec<String>, volume_level: u8, shuffle: bool) -> Self {
        Settings {
            music_paths,
            volume_level,
            shuffle,
        }
    }

    pub fn get_music_paths(&self) -> &Vec<String> {
        &self.music_paths
    }

    pub fn get_volume_level(&self) -> u8 {
        self.volume_level
    }

    pub fn is_shuffle_enabled(&self) -> bool {
        self.shuffle
    }
}

impl Debug for Settings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Settings")
            .field("music_paths", &self.music_paths)
            .field("volume_level", &self.volume_level)
            .field("shuffle", &self.shuffle)
            .finish()
    }
}