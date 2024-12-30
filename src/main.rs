// Local mods
mod app;
mod tabs;

// Use mod
use app::App;
use std::io;
use ratatui;

// Developed Main Termianl window initialization
//
//
//
fn main() -> io::Result<()> {
    // Initialize the terminal for ratatui
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
