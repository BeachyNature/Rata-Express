// src/waether_app.rs
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    text::{Line,Text},
    symbols::border,
    style::Stylize,
    Frame,
};

#[derive(Debug, Default)]
pub struct WeatherApp;

impl WeatherApp {
    // Render the weather app
    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let title = Line::from(" Weather App ".bold());
        let block = Block::default()
            .title(title.centered())
            .borders(Borders::ALL)
            .border_set(border::THICK);
        
        // Placeholder text
        let paragraph = Paragraph::new(Text::raw("Welcome to the Weather App! Here is some content."))
            .block(block);
        frame.render_widget(paragraph, area);
    }
}
