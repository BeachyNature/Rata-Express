use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    widgets::{Paragraph, Borders, Block, Tabs},
    layout::{Layout, Constraint, Direction},
    symbols::border,
    style::Stylize,
    text::Line,
    DefaultTerminal,
    Frame,
};

// Import local app
use crate::{
    tabs::{weather_app}
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    tabs: Vec<String>,
    current_tab: usize,
    weather_app: weather_app::WeatherApp,
}

impl App {
    // Draw current terminal frames
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // Draw visible window
    fn draw(&self, frame: &mut Frame) {
        let title = Line::from("Very Epic Child Block".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".red().bold(),
        ]);

        // Define the main layout
        let area = frame.area();
        let parent_block = Block::default()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        frame.render_widget(parent_block, area);

        // Child block layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(1),  // Title
                    Constraint::Percentage(2),  // Tabs
                    Constraint::Percentage(95), // Widget Space
                    Constraint::Percentage(2),  // Button Layout
                ]
                .as_ref(),
            )
            .split(area);

        // Setup tabs
        // let tab_titles: Vec<Line> = self.tabs
        let tabs = Tabs::new(vec!["Tab 1".blue(), "Tab 2".blue()])
            .padding("", "")
            .divider(" ");
        frame.render_widget(tabs, layout[1]);

        // Content for the selected tab
        let content = match self.current_tab {
            0 => self.weather_app.render(frame, layout[2]),
            1 => {
                let paragraph = Paragraph::new("EPIC TEST")
                    .block(Block::default().title("Content").borders(Borders::ALL));
                frame.render_widget(paragraph, layout[2]);
            }
            2 => {
                let paragraph = Paragraph::new("Very cool")
                    .block(Block::default().title("Content").borders(Borders::ALL));
                frame.render_widget(paragraph, layout[2]);
            },
            _ => {},
        };
    }

    // Get the key press event for user wanting to close the window
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
   
    // Capture the key press that the user presses
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('1') => {
                self.current_tab = 0;
            }
            KeyCode::Char('2') => {
                self.current_tab = 1;
            }
            _ => {},
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

