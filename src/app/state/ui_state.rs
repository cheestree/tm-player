use crate::app::app_screen::{AppScreen, Overlay};
use crate::app::screens::help::HelpScreenState;
use crate::app::screens::main::MainScreenState;
use crate::app::screens::settings::SettingsScreenState;

/// Which panel currently has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Sidebar,
    MainContent,
}

pub struct UIState {
    pub side_bar: bool,
    pub show_debug: bool,
    pub focus: Focus,
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
            focus: Focus::MainContent,
            screen: AppScreen::Main,
            overlay: None,
            main: MainScreenState::default(),
            settings: SettingsScreenState::default(),
            help: HelpScreenState::default(),
        }
    }

    pub fn toggle_sidebar(&mut self) {
        self.side_bar = !self.side_bar;
    }

    pub fn toggle_debug(&mut self) {
        self.show_debug = !self.show_debug;
    }

    /// Toggle focus between sidebar and main content
    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Sidebar => Focus::MainContent,
            Focus::MainContent => Focus::Sidebar,
        };
    }

    /// Set focus to sidebar
    pub fn focus_sidebar(&mut self) {
        self.focus = Focus::Sidebar;
    }

    /// Set focus to main content
    pub fn focus_main(&mut self) {
        self.focus = Focus::MainContent;
    }

    /// Check if sidebar has focus
    pub fn is_sidebar_focused(&self) -> bool {
        self.focus == Focus::Sidebar
    }

    /// Check if main content has focus
    pub fn is_main_focused(&self) -> bool {
        self.focus == Focus::MainContent
    }
}
