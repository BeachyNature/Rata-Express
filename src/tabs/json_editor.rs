use std::collections::HashMap;

use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    text::{Line,Text},
    symbols::border,
    style::Stylize,
    Frame,
};

enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Key,
    Value,
}

#[derive(Debug, Default)]
pub struct JsonApp {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub is_editing: Option<CurrentlyEditing>,    
}


impl JsonApp {
    // Render the json widget window
    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(5),
                    Constraint::Percentage(45),
                    Constraint::Percentage(45),
                    Constraint::Percentage(5),
                ]
                .as_ref(),
            )
            .split(area);

        // Text layouts
        let intro_text = Text::raw("Welcome to the JSON Creator App!");
        let title = Line::from(" Json Creator App ".bold());
        let instruct = Line::from(vec![
            " Edit/View ".into(),
            "<E>".blue(),
            " Save ".into(),
            "<S>".blue().bold(),
        ]);

        // Main block
        let main_block = Block::default()
            .title(title.centered())
            .title_bottom(instruct.centered())
            .borders(Borders::ALL)
            .border_set(border::THICK);
       
        let key_block = Block::default()
            .title("Key")
            .borders(Borders::ALL);


        // Paragraphs
        let paragraph = Paragraph::new(intro_text)
            .block(main_block);
        let key_paragraph = Paragraph::new(self.key_input.clone())
            .block(key_block);

        // Render widgets
        frame.render_widget(key_paragraph, layout[1]);
        frame.render_widget(paragraph, area);
    }

    pub fn save_key_value(&mut self) {
        //Save new values to hashmap
        self.pairs.insert(self.key_input.clone(), self.value_input.clone());
        
        // Reset key and value statuses
        self.key_input = String::new();
        self.value_input = String::new();
        self.is_editing = None;
    }
}

