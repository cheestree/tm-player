use std::fmt::Debug;
use crate::app::app_screen::{AppScreen, AppState, Overlay};
use crate::app::{help_screen, main_screen};
use crate::app::settings_screen;
use crate::settings::settings::Settings;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::sync::{Arc, Mutex};
use cli_log::{info, init_cli_log};
use ratatui::prelude::Widget;
use ratatui::widgets::Paragraph;
use rodio::{OutputStream, OutputStreamBuilder, Sink};
use crate::{parser, track};

pub struct AudioState {
    pub tracks: Vec<track::track::Track>,
    pub selected_track: usize,
}

pub struct UIState {
    pub side_bar: bool,
    pub screen: AppState,
    pub show_debug: bool,
}

pub struct App {
    pub settings: Settings,
    pub audio: AudioState,
    pub ui: UIState,
    pub settings_index: usize,
    exit: bool,
    _stream: OutputStream,
    audio_sink: Arc<Mutex<Sink>>,
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
        for path in &settings.music_paths {
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
        match self.ui.screen.screen {
            AppScreen::Main => main_screen::render(frame.area(), frame.buffer_mut(), &self),
            AppScreen::Help => help_screen::render(frame.area(), frame.buffer_mut(), &self),
        }

        // Render overlay if present
        if let Some(overlay) = &self.ui.screen.overlay {
            match overlay {
                Overlay::Settings => settings_screen::render(frame.area(), frame.buffer_mut(), &self),
            }
        }

        if self.ui.show_debug {
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
        if let Some(overlay) = &self.ui.screen.overlay {
            match overlay {
                Overlay::Settings => {
                    match key_event.code {
                        KeyCode::Char('l') => {
                            if let Ok(settings) = parser::parser::parse_settings("settings.json") {
                                self.settings = settings;
                                self.settings_index = 0;
                            }
                        },
                        KeyCode::Char('m') => {
                            self.ui.screen.overlay = None
                        },
                        _ => {}
                    }
                }
            }
            return; // If an overlay is active, do not process base screen keys
        }
        match key_event.code {
            KeyCode::Char('s') => self.toggle_sidebar(),
            KeyCode::Char('h') => {
                self.ui.screen.overlay = Option::from(Overlay::Settings)
            },
            KeyCode::Char('p') => {
                self.toggle_pause()
            }
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn toggle_pause(&mut self) {
        let sink = self.audio_sink.lock().unwrap();
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }

    fn toggle_sidebar(&mut self) {
        self.ui.side_bar = !self.ui.side_bar;
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
        Self {
            settings,
            audio: AudioState {
                tracks,
                selected_track: 0,
            },
            ui: UIState {
                side_bar: true,
                screen: AppState {
                    screen: AppScreen::Main,
                    overlay: None,
                },
                show_debug: false,
            },
            settings_index: 0,
            exit: false,
            _stream: OutputStreamBuilder::open_default_stream().expect("open default audio stream"),
            audio_sink: Arc::new(Mutex::new(Sink::connect_new(&OutputStreamBuilder::open_default_stream().expect("open default audio stream").mixer()))),
        }
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("settings", &self.settings)
            .field("settings_index", &self.settings_index)
            .field("side_bar", &self.ui.side_bar)
            .field("screen", &self.ui.screen)
            .field("tracks_count", &self.audio.tracks.len())
            .field("selected_track", &self.audio.selected_track)
            .field("show_debug", &self.ui.show_debug)
            .field("exit", &self.exit)
            .finish()
    }
}