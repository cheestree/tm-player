use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Keymap {
    // Playback actions
    Play,
    Pause,
    NextTrack,
    PreviousTrack,
    VolumeUp,
    VolumeDown,
    Rescan,
    // Global UI actions
    ToggleSidebar,
    ToggleDebug,
    OpenSettings,
    Quit,
}

impl Keymap {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Keymap::Play => "Play",
            Keymap::Pause => "Pause",
            Keymap::NextTrack => "Next Track",
            Keymap::PreviousTrack => "Previous Track",
            Keymap::VolumeUp => "Volume Up",
            Keymap::VolumeDown => "Volume Down",
            Keymap::Rescan => "Rescan Tracks",
            Keymap::ToggleSidebar => "Toggle Sidebar",
            Keymap::ToggleDebug => "Toggle Debug",
            Keymap::OpenSettings => "Open Settings",
            Keymap::Quit => "Quit",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    music_paths: Vec<String>,
    volume_level: u8,
    shuffle: bool,
    #[serde(with = "keymap_serde")]
    keymap: HashMap<KeyCode, Keymap>,
}

// Custom serialization for HashMap<KeyCode, Keymap>
mod keymap_serde {
    use super::*;
    use serde::{Deserializer, Serializer, Deserialize};

    #[derive(Serialize, Deserialize)]
    struct KeymapEntry {
        action: Keymap,
        key: KeyCodeWrapper,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum KeyCodeWrapper {
        Char(char),
        Special(String),
    }

    impl From<KeyCode> for KeyCodeWrapper {
        fn from(kc: KeyCode) -> Self {
            match kc {
                KeyCode::Char(c) => KeyCodeWrapper::Char(c),
                KeyCode::Enter => KeyCodeWrapper::Special("Enter".to_string()),
                KeyCode::Left => KeyCodeWrapper::Special("Left".to_string()),
                KeyCode::Right => KeyCodeWrapper::Special("Right".to_string()),
                KeyCode::Up => KeyCodeWrapper::Special("Up".to_string()),
                KeyCode::Down => KeyCodeWrapper::Special("Down".to_string()),
                KeyCode::Backspace => KeyCodeWrapper::Special("Backspace".to_string()),
                KeyCode::Tab => KeyCodeWrapper::Special("Tab".to_string()),
                KeyCode::Esc => KeyCodeWrapper::Special("Esc".to_string()),
                _ => KeyCodeWrapper::Special("Unknown".to_string()),
            }
        }
    }

    impl From<KeyCodeWrapper> for KeyCode {
        fn from(wrapper: KeyCodeWrapper) -> Self {
            match wrapper {
                KeyCodeWrapper::Char(c) => KeyCode::Char(c),
                KeyCodeWrapper::Special(s) => match s.as_str() {
                    "Enter" => KeyCode::Enter,
                    "Left" => KeyCode::Left,
                    "Right" => KeyCode::Right,
                    "Up" => KeyCode::Up,
                    "Down" => KeyCode::Down,
                    "Backspace" => KeyCode::Backspace,
                    "Tab" => KeyCode::Tab,
                    "Esc" => KeyCode::Esc,
                    _ => KeyCode::Char(' '), // fallback
                },
            }
        }
    }

    pub fn serialize<S>(
        keymap: &HashMap<KeyCode, Keymap>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries: Vec<KeymapEntry> = keymap
            .iter()
            .map(|(key, action)| KeymapEntry {
                action: action.clone(),
                key: (*key).into(),
            })
            .collect();
        entries.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<KeyCode, Keymap>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries = Vec::<KeymapEntry>::deserialize(deserializer)?;
        Ok(entries
            .into_iter()
            .map(|entry| (entry.key.into(), entry.action))
            .collect())
    }
}

impl Settings {
    pub fn new(music_paths: Vec<String>, volume_level: u8, shuffle: bool, keymap: HashMap<KeyCode, Keymap>) -> Self {
        Settings {
            music_paths,
            volume_level,
            shuffle,
            keymap,
        }
    }

    pub fn get_keymap(&self) -> &HashMap<KeyCode, Keymap> {
        &self.keymap
    }

    pub fn get_action(&self, keycode: &KeyCode) -> Option<&Keymap> {
        self.keymap.get(keycode)
    }

    pub fn set_keybind(&mut self, keycode: KeyCode, action: Keymap) {
        self.keymap.insert(keycode, action);
    }
    
    pub fn remove_keybind(&mut self, keycode: &KeyCode) {
        self.keymap.remove(keycode);
    }
    
    pub fn find_key_for_action(&self, action: &Keymap) -> Option<KeyCode> {
        self.keymap
            .iter()
            .find(|(_, a)| *a == action)
            .map(|(k, _)| *k)
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

    pub fn set_music_paths(&mut self, paths: Vec<String>) {
        self.music_paths = paths;
    }

    pub fn add_music_path(&mut self, path: String) {
        self.music_paths.push(path);
    }

    pub fn set_volume_level(&mut self, level: u8) {
        self.volume_level = level.min(100);
    }

    pub fn set_shuffle(&mut self, enabled: bool) {
        self.shuffle = enabled;
    }

    pub fn toggle_shuffle(&mut self) {
        self.shuffle = !self.shuffle;
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