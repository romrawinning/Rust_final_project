mod system;
use std::fs;
use iced::widget::{button, column, row, scrollable, text, text_input};
use iced::{Alignment, Length, Element, Sandbox, Settings, Theme, Command};

pub struct Gui_Viewer {
    file_name: String,
    keyword: String,
    content: Vec<String>,
}
#[derive(Debug, Clone)]
pub enum Message {
    File_name_change(String),
    Kw_change(String),
    OpenFile,
    Search,
}

fn read_lines (path: &str) -> Result<Vec<String>, String> {
    fs::read_to_string(path)
        .map(|s| s.lines().map(|line| line.to_string()).collect())
        .map_err(|e| format!("Cannot open '{}': {}", path, e)
                 }

fn main() {
    
}
