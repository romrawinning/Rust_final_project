
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

 fn view(&self) -> Element<Message> {
        let lines: Vec<Element<Message>> = self
            .filtered
            .iter()
            .map(|line| text(line).into())
            .collect();

        let mut page = column![
            text("Text / Log File Viewer").size(28),
            row![
                text_input("File path...", &self.file_name)
                    .on_input(Message::File_name_change),
                button("Open").on_press(Message::OpenFile),
            ]
            .spacing(10),
            row![
                text_input("Keyword...", &self.keyword).on_input(Message::Kw_change),
                button("Search").on_press(Message::Search),
            ]
            .spacing(10),
            text(format!(
                "Matches: {}",
                count_matches(&self.content, &self.keyword)
            )),
        ]
        .spacing(10)
        .padding(20);

        if !self.error.is_empty() {
            page = page.push(text(&self.error));
        }

        page.push(scrollable(column(lines).spacing(2))).into()
    }
}

fn main() -> iced::Result {
    match system::test_database() {
        Ok(()) => println!("Database connected successfully!"),
        Err(e) => eprintln!("Database connection failed: {}", e),
    }

    Gui_Viewer::run(Settings::default())
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
