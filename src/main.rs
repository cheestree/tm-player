mod app;
mod settings;
mod track;

use std::io;

use crate::app::app::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
