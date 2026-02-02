#[derive(Debug, Default)]
pub enum AppScreen {
    #[default]
    Main,
    Help
}

#[derive(Debug)]
pub enum Overlay {
    Settings,
}