use crate::app::app_screen::{AppScreen, Overlay};
use crate::app::state::{AudioState, UIState};
use crate::app::screens::{help, main, settings};
use crate::settings::settings::{Keymap, Settings};
use crate::track;
use cli_log::init_cli_log;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::Widget;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};
use rodio::OutputStreamBuilder;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::sync::{Arc, Mutex};


pub struct App {
    pub settings: Settings,
    pub audio: AudioState,
    pub ui: UIState,
    exit: bool,
}

impl App {
    fn default_keymap() -> HashMap<KeyCode, Keymap> {
        let mut map = HashMap::new();
        // Playback actions
        map.insert(KeyCode::Enter, Keymap::Play);
        map.insert(KeyCode::Char(' '), Keymap::Pause);
        map.insert(KeyCode::Right, Keymap::NextTrack);
        map.insert(KeyCode::Left, Keymap::PreviousTrack);
        map.insert(KeyCode::Up, Keymap::VolumeUp);
        map.insert(KeyCode::Down, Keymap::VolumeDown);
        map.insert(KeyCode::Char('r'), Keymap::Rescan);
        // Sidebar actions
        map.insert(KeyCode::Char('l'), Keymap::SelectPlaylist);
        map.insert(KeyCode::Char('c'), Keymap::CreatePlaylist);
        map.insert(KeyCode::Char('x'), Keymap::DeletePlaylist);
        // Global UI actions
        map.insert(KeyCode::Tab, Keymap::ToggleFocus);
        map.insert(KeyCode::Char('d'), Keymap::ToggleSidebar);
        map.insert(KeyCode::Char('p'), Keymap::ToggleDebug);
        map.insert(KeyCode::Char('s'), Keymap::OpenSettings);
        map.insert(KeyCode::Char('q'), Keymap::Quit);
        map
    }

    pub fn load_settings() -> Settings {
        let path = "settings.json";

        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&contents) {
                return settings;
            }
        }

        let default_settings = Settings::new(
            vec![],
            50,
            false,
            Self::default_keymap()
        );

        if let Ok(json) = serde_json::to_string_pretty(&default_settings) {
            let _ = std::fs::write(path, json);
        }

        default_settings
    }

    pub fn save_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = "settings.json";
        let json = serde_json::to_string_pretty(&self.settings)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_tracks(settings: &Settings) -> Vec<track::track::Track> {
        if let Ok(cached) = Self::load_tracks_from_cache() {
            return cached;
        }

        Self::scan_tracks(settings)
    }

    fn load_tracks_from_cache() -> Result<Vec<track::track::Track>, Box<dyn std::error::Error>> {
        let cache_path = "tracks_cache.json";
        let contents = std::fs::read_to_string(cache_path)?;
        let tracks: Vec<track::track::Track> = serde_json::from_str(&contents)?;
        Ok(tracks)
    }

    pub fn scan_tracks(settings: &Settings) -> Vec<track::track::Track> {
        let mut tracks = Vec::new();
        for path in settings.get_music_paths() {
            let track_paths = track::utils::get_audio_file_paths_in_directory(path);
            tracks.extend(track_paths.iter().map(|t| track::track::Track::new(t)).collect::<Vec<track::track::Track>>());
        }

        let _ = Self::save_tracks_to_cache(&tracks);

        tracks
    }

    fn save_tracks_to_cache(tracks: &[track::track::Track]) -> Result<(), Box<dyn std::error::Error>> {
        let cache_path = "tracks_cache.json";
        let json = serde_json::to_string_pretty(tracks)?;
        std::fs::write(cache_path, json)?;
        Ok(())
    }

    pub fn rescan_tracks(&mut self) {
        self.audio.tracks = Self::scan_tracks(&self.settings);
        // Reset selected track if out of bounds
        if self.ui.main.selected_track() >= self.audio.tracks.len() {
            self.ui.main.select_previous();
        }
    }

    pub fn load_playlists(&self) -> Vec<Vec<usize>> {
        let path = "playlists.json";
        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Ok(playlists) = serde_json::from_str::<Vec<Vec<usize>>>(&contents) {
                return playlists;
            }
        }
        vec![]
    }

    pub fn save_playlists(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = "playlists.json";
        let json = serde_json::to_string_pretty(&self.audio.playlists)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // Always render base screen
        match self.ui.screen {
            AppScreen::Main => main::render(frame.area(), frame.buffer_mut(), &self),
            AppScreen::Help => help::render(frame.area(), frame.buffer_mut(), &self),
        }

        // Render overlay if present
        if let Some(overlay) = &self.ui.overlay {
            match overlay {
                Overlay::Settings => settings::render(frame.area(), frame.buffer_mut(), &self),
            }
        }

        if self.ui.show_debug() {
            Paragraph::new(format!("{self:#?}")).render(frame.area(), frame.buffer_mut());
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // Handle overlays first
        if let Some(overlay) = &self.ui.overlay {
            match overlay {
                Overlay::Settings => settings::handle_key_event(self, key_event),
            }
            return;
        }

        // Check if key is bound to any global action
        if let Some(action) = self.settings.get_action(&key_event.code) {
            match action {
                Keymap::ToggleFocus => {
                    if self.ui.side_bar() {
                        self.ui.toggle_focus();
                    }
                    return;
                }
                Keymap::ToggleSidebar => {
                    self.ui.toggle_sidebar();
                    if !self.ui.side_bar() {
                        self.ui.focus_main();
                    }
                    return;
                }
                Keymap::ToggleDebug => {
                    self.ui.toggle_debug();
                    return;
                }
                Keymap::OpenSettings => {
                    self.ui.overlay = Some(Overlay::Settings);
                    return;
                }
                Keymap::Quit => {
                    self.exit();
                    return;
                }
                _ => {}
            }
        }

        // Delegate to screen
        match self.ui.screen {
            AppScreen::Main => main::handle_key_event(self, key_event),
            AppScreen::Help => help::handle_key_event(self, key_event),
        }
    }


    fn exit(&mut self) {
        // Save playlists before exiting
        let _ = self.audio.save_playlists("playlists.json");
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        init_cli_log!();
        let settings = App::load_settings();
        let tracks = App::load_tracks(&settings);

        let audio_stream = OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let audio_sink = Arc::new(Mutex::new(rodio::Sink::connect_new(&audio_stream.mixer())));
        
        // Try to load playlists, otherwise create default
        let audio = if let Ok(playlist_data) = AudioState::load_playlists("playlists.json") {
            AudioState::with_playlists(tracks, audio_stream, audio_sink, playlist_data)
        } else {
            AudioState::new(tracks, audio_stream, audio_sink)
        };

        Self {
            settings,
            audio,
            ui: UIState::new(),
            exit: false,
        }
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("settings", &self.settings)
            .field("settings_index", &self.ui.settings.selected_index())
            .field("side_bar", &self.ui.side_bar())
            .field("screen", &self.ui.screen)
            .field("tracks_count", &self.audio.tracks.len())
            .field("selected_track", &self.ui.main.selected_track())
            .field("show_debug", &self.ui.show_debug())
            .field("exit", &self.exit)
            .finish()
    }
}