mod system;
use iced::widget::{button, column, row, scrollable, text, text_input};
use iced::{Element, Sandbox, Settings, Theme};
use system::{count_matches, filter_lines, read_lines};

#[derive(Debug, Clone)]
pub enum Message {
    File_name_change(String),
    Kw_change(String),
    OpenFile,
    Search,
    LoadHistory,
    ToggleTheme,
    GoTutorial,
    GoSearch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Search,
    History,
    Tutorial,
}

pub struct Gui_Viewer {
    file_name: String,
    keyword: String,
    content: Vec<String>,
    filtered: Vec<String>,
    error: String,
    history: Vec<system::SearchHistory>,
    theme: Theme,
    page: Page,
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
            history: Vec::new(),
            theme: Theme::Light,
            page: Page::Search,
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
                let matches = self.filtered.len();

                match system::save_search_history(&self.file_name, &self.keyword, matches) {
                    Ok(()) => self.error.clear(),
                    Err(e) => self.error = format!("Failed to save search history: {}", e),
                }
            }
            Message::LoadHistory => {
                // Load search history from the database
                match system::get_search_history() {
                    Ok(history) => {
                        self.history = history;
                        self.error.clear();
                        self.page = Page::History; // Switch to the history page after loading
                    }
                    Err(e) => self.error = format!("Failed to load search history: {}", e),
                }
            }
            Message::ToggleTheme => {
                self.theme = match self.theme.clone() {
                    Theme::Light => Theme::Dark,
                    _ => Theme::Light,
                };
            }
            Message::GoTutorial => {
                self.page = Page::Tutorial;
            }
            Message::GoSearch => {
                self.page = Page::Search;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.page {
            Page::Search => {
                let lines: Vec<Element<Message>> =
                    self.filtered.iter().map(|line| text(line).into()).collect();

                column![
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
                    button("Search History").on_press(Message::LoadHistory),
                    button("Toggle Theme").on_press(Message::ToggleTheme),
                    button("?").on_press(Message::GoTutorial),
                    text(&self.error),
                    scrollable(column(lines).spacing(2)),
                ]
                .spacing(10)
                .padding(20)
                .into()
            }

            Page::History => {
                let mut page = column![
                    text("Search History").size(28),
                    button("Back").on_press(Message::GoSearch),
                ]
                .spacing(10)
                .padding(20);

                for item in &self.history {
                    page = page.push(text(format!(
                        "{} | {} | {} | {} | {}",
                        item.id, item.file_path, item.keyword, item.match_count, item.created_at
                    )));
                }

                scrollable(page).into()
            }

            Page::Tutorial => column![
                text("Tutorial").size(28),
                text("1. Enter your file path."),
                text("2. Click Open to load your file."),
                text("3. Enter a keyword."),
                text("4. Click Search to find matching lines."),
                text("5. Click Load Search History to see your history."),
                text("6. Click Toggle Theme to switch between light and dark mode."),
                button("Back").on_press(Message::GoSearch),
            ]
            .spacing(10)
            .padding(20)
            .into(),
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
    
}

fn main() -> iced::Result {
    match system::test_database() {
        Ok(()) => println!("Database connected successfully!"),
        Err(e) => eprintln!("Database connection failed: {}", e),
    }

    Gui_Viewer::run(Settings::default())
}
