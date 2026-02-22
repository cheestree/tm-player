use crate::app::app::App;
use crate::app::app_screen::TrackAction;

/// Get available actions for the current track/playlist context
pub fn get_available_actions(app: &App) -> Vec<TrackAction> {
    let mut actions = vec![TrackAction::AddToPlaylist];

    // Only allow remove/move if not in "All Tracks" playlist
    if app.ui.main.selected_playlist > 0 {
        actions.push(TrackAction::RemoveFromPlaylist);

        // Only allow moving when not sorted (moving in sorted view doesn't make sense)
        if app.ui.main.sorted_by.is_none() {
            actions.push(TrackAction::MoveUp);
            actions.push(TrackAction::MoveDown);
        }
    }

    actions
}
