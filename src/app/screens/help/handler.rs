use crate::app::app::App;
use crate::app::app_screen::AppScreen;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events specific to the help screen
pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            // Return to main screen
            app.ui.screen = AppScreen::Main;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Scroll down
            app.ui.help.scroll_down();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Scroll up
            app.ui.help.scroll_up();
        }
        _ => {}
    }
}
