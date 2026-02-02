use crate::app::app_screen::{AppScreen, Overlay};
use crate::app::state::{AudioState, UIState};
use crate::app::screens::{help, main, settings};
use crate::settings::settings::Settings;
use crate::track;
use cli_log::init_cli_log;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::Widget;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};
use rodio::OutputStreamBuilder;
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
    pub fn load_settings() -> Settings {
        let path = "settings.json";
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str::<serde_json::Value>(&contents) {
                    Ok(parsed) => {
                        let music_paths = parsed["music_paths"]
                            .as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect();
                        let volume_level = parsed["volume_level"].as_u64().unwrap_or(50) as u8;
                        let shuffle = parsed["shuffle"].as_bool().unwrap_or(false);
                        Settings::new(music_paths, volume_level, shuffle)
                    }
                    Err(_) => Settings::new(vec![], 50, false),
                }
            }
            Err(_) => {
                let default = Settings::new(vec![], 50, false);
                let json = serde_json::json!({
                    "music_paths": [],
                    "volume_level": 50,
                    "shuffle": false
                });
                let _ = std::fs::write(path, serde_json::to_string_pretty(&json).unwrap());
                default
            }
        }
    }

    pub fn load_tracks(settings: &Settings) -> Vec<track::track::Track> {
        let mut tracks = Vec::new();
        for path in settings.get_music_paths() {
            let track_paths = track::utils::get_audio_file_paths_in_directory(path);
            tracks.extend(track_paths.iter().map(|t| track::track::Track::new(t)).collect::<Vec<track::track::Track>>());
        }
        tracks
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

        // Global keys
        match key_event.code {
            KeyCode::Char('d') => self.ui.toggle_sidebar(),
            KeyCode::Char('p') => self.ui.toggle_debug(),
            KeyCode::Char('s') => self.ui.overlay = Some(Overlay::Settings),
            KeyCode::Char('q') => self.exit(),
            _ => {},
        }

        // Delegate to screen
        match self.ui.screen {
            AppScreen::Main => main::handle_key_event(self, key_event),
            AppScreen::Help => help::handle_key_event(self, key_event),
        }
    }


    fn exit(&mut self) {
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
        
        Self {
            settings,
            audio: AudioState::new(tracks, audio_stream, audio_sink),
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