use std::fs;
use std::io;
use iced::widget::{button, column, row, text, text_input, scrollable};
use iced::{Command, Element};


struct Gui_Viewer {
    file_name: String,
    keyword: String,
    content: Vec<String>,
}

enum Message {
    File_name_change(String),
    Kw_change(String),
    OpenFile,
    Search,
}

fn read_lines (path: &str) -> Result<Vec<String>, String> {
    fs::read_to_string(path)
    .map(|s| s.lines().map(|line| line.to_string()).collect())
    .map_err(|e| format! ("Cannot open '{}' : {}", path, e))
}

fn main() {
    


}
