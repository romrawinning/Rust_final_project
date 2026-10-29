
mod system;

use std::fs;

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
