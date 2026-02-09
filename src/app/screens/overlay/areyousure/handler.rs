use crate::app::app::App;
use crate::app::app_screen::ConfirmAction;
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events for the AreYouSure overlay
pub fn handle_key_event(
    app: &mut App,
    key_event: KeyEvent,
    _title: &str,
    _description: &Option<String>,
    action: &ConfirmAction,
) {
    match key_event.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
            // User confirmed - execute the action
            execute_action(app, action);
            app.ui.overlay = None;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            // User cancelled - just close overlay
            app.ui.overlay = None;
        }
        _ => {}
    }
}

/// Execute the confirmed action
fn execute_action(app: &mut App, action: &ConfirmAction) {
    match action {
        ConfirmAction::DeletePlaylist(name) => {
            app.audio.delete_playlist(name);
        } // Add more actions as needed
    }
}
