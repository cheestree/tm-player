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

#[derive(Debug)]
pub struct AppState {
    pub screen: AppScreen,
    pub overlay: Option<Overlay>,
}