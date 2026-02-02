use crate::app::app_screen::{AppScreen, Overlay};
use crate::app::screens::help::HelpScreenState;
use crate::app::screens::main::MainScreenState;
use crate::app::screens::settings::SettingsScreenState;

pub struct UIState {
    side_bar: bool,
    show_debug: bool,
    pub screen: AppScreen,
    pub overlay: Option<Overlay>,
    pub main: MainScreenState,
    pub settings: SettingsScreenState,
    pub help: HelpScreenState,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            side_bar: true,
            show_debug: false,
            screen: AppScreen::Main,
            overlay: None,
            main: MainScreenState::default(),
            settings: SettingsScreenState::default(),
            help: HelpScreenState::default(),
        }
    }

    pub fn side_bar(&self) -> bool {
        self.side_bar
    }

    pub fn toggle_sidebar(&mut self) {
        self.side_bar = !self.side_bar;
    }

    pub fn show_debug(&self) -> bool {
        self.show_debug
    }

    pub fn toggle_debug(&mut self) {
        self.show_debug = !self.show_debug;
    }
}
