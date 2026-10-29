
mod system;
use system::{count_matches, filter_lines, read_lines};
use std::fs;
use iced::widget::{button, column, row, scrollable, text, text_input};
use iced::{Element, Sandbox, Settings};

#[derive(Debug, Clone)]
pub enum Message {
    File_name_change(String),
    Kw_change(String),
    OpenFile,
    Search,
}

pub struct Gui_Viewer {
    file_name: String,
    keyword: String,
    content: Vec<String>,
    filtered: Vec<String>,
    error: String,
}

impl Sandbox for Gui_Viewer {
    type Message = Message;

    fn new() -> Self {
        Gui_Viewer {
            file_name: String::new(),
            keyword: String::new(),
            content: Vec::new(),
            filtered: Vec::new(),
            error: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Text / Log File Viewer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::File_name_change(s) => self.file_name = s,
            Message::Kw_change(s) => self.keyword = s,
            Message::OpenFile => match read_lines(&self.file_name) {
                Ok(lines) => {
                    self.content = lines;
                    self.filtered = self.content.clone();
                    self.error.clear();
                }
                Err(e) => self.error = e,
            },
            Message::Search => {
                self.filtered = filter_lines(&self.content, &self.keyword);
            }
        }
    }

fn read_lines(path: &str) -> Result<Vec<String>, String> {
    fs::read_to_string(path)
        .map(|s| s.lines().map(|line| line.to_string()).collect())
        .map_err(|e| format!("Cannot open '{}': {}", path, e))
}

fn main() {
    match system::test_database() {
        Ok(()) => {
            println!("Database connected successfully!");
        }
        Err(e) => {
            eprintln!("Database connection failed: {}", e);
        }
    }
}
