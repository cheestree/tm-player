/// Represents the different screens in the application.
#[derive(Debug, Default)]
pub enum AppScreen {
    #[default]
    Main,
    Help
}

/// Represents the different overlays in the application.
#[derive(Debug)]
pub enum Overlay {
    Settings,
}