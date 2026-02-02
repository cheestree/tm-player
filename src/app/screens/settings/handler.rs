use crate::app::app::App;
use crossterm::event::{KeyCode, KeyEvent};
use crate::parser;

/// Handle key events specific to the settings screen
pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.ui.settings.select_next(3); // TODO: Make dynamic based on menu items
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.ui.settings.select_previous();
        }
        KeyCode::Char('l') => {
            // Reload settings from file
            if let Ok(settings) = parser::parser::parse_settings("settings.json") {
                app.settings = settings;
                app.ui.settings.reset();
            }
        }
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('m') => {
            // Close settings overlay
            app.ui.overlay = None;
        }
        _ => {}
    }
}
