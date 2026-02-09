/// Represents the different screens in the application.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum AppScreen {
    #[default]
    Main,
    Help,
}

/// Action to execute when user confirms in AreYouSure overlay
#[derive(Debug, Clone)]
pub enum ConfirmAction {
    DeletePlaylist(String),
}

/// Represents the different overlays in the application.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Overlay {
    Settings,
    AreYouSure {
        title: String,
        description: Option<String>,
        action: ConfirmAction,
    },
    DeletePlaylist,
}
