// Local mods
mod app;
mod tabs;

// Use mod
use app::App;
use std::io;
use ratatui;

/// Developed Main Termianl window initialization
///
/// WIP
///
fn main() -> io::Result<()> {
    // Setup terminal window 
    let mut app = App::default();
    let mut terminal = ratatui::init();
    terminal.clear()?;
    
    // Run App in default state 
    let _ = app.run(&mut terminal);
    ratatui::restore();
    Ok(())
}
