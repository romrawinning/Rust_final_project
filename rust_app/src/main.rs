mod system;

use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme, executor};
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
    Export,
    ConfirmExport,
    CancelExport,
    FileLoaded(Result<Vec<String>, String>),
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
    confirm_export: bool,
    is_loading: bool,
}

impl Application for Gui_Viewer {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Gui_Viewer {
                file_name: String::new(),
                keyword: String::new(),
                content: Vec::new(),
                filtered: Vec::new(),
                error: String::new(),
                history: Vec::new(),
                theme: Theme::Light,
                page: Page::Search,
                confirm_export: false,
                is_loading: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Text / Log File Viewer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::File_name_change(s) => {
                self.file_name = s;
            }

            Message::Kw_change(s) => {
                self.keyword = s;
            }

            Message::OpenFile => {
                self.is_loading = true;

                let file_name = self.file_name.clone();

                return Command::perform(
                    async move { read_lines(&file_name) },
                    Message::FileLoaded,
                );
            }

            Message::FileLoaded(result) => {
                self.is_loading = false;

                match result {
                    Ok(lines) => {
                        self.content = lines;
                        self.filtered = self.content.clone();
                        self.error.clear();
                    }
                    Err(e) => {
                        self.error = e;
                    }
                }
            }

            Message::Search => {
                self.filtered = filter_lines(&self.content, &self.keyword);

                let matches = self.filtered.len();

                match system::save_search_history(&self.file_name, &self.keyword, matches) {
                    Ok(()) => self.error.clear(),
                    Err(e) => {
                        self.error = format!("Failed to save search history: {}", e);
                    }
                }
            }

            Message::LoadHistory => match system::get_search_history() {
                Ok(history) => {
                    self.history = history;
                    self.error.clear();
                    self.page = Page::History;
                }
                Err(e) => {
                    self.error = format!("Failed to load search history: {}", e);
                }
            },

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

            Message::Export => {
                self.confirm_export = true;
            }

            Message::CancelExport => {
                self.confirm_export = false;
            }

            Message::ConfirmExport => match system::export_lines("result.txt", &self.filtered) {
                Ok(()) => {
                    let path = std::env::current_dir().unwrap_or_default();

                    self.error = format!("Export saved to: {}", path.join("result.txt").display());

                    self.confirm_export = false;
                }
                Err(e) => {
                    self.error = format!("Export failed: {}", e);
                    self.confirm_export = false;
                }
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.page {
            Page::Search => {
                if self.confirm_export {
                    return container(
                        column![
                            text("Do you want to export the results?").size(24),
                            row![
                                button("Yes").on_press(Message::ConfirmExport),
                                button("Cancel").on_press(Message::CancelExport),
                            ]
                            .spacing(15),
                        ]
                        .spacing(20)
                        .align_items(Alignment::Center),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into();
                }

                let lines: Vec<Element<Message>> =
                    self.filtered.iter().map(|line| text(line).into()).collect();

                let mut page = column![
                    // Header
                    row![
                        text("Text / Log File Viewer").size(28),
                        Space::with_width(Length::Fill),
                        button("Toggle Theme").on_press(Message::ToggleTheme),
                        button("Search History").on_press(Message::LoadHistory),
                        button("Export TXT").on_press(Message::Export),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                    // File path
                    row![
                        text_input("File path...", &self.file_name)
                            .on_input(Message::File_name_change),
                        button(if self.is_loading {
                            "Loading..."
                        } else {
                            "Open"
                        })
                        .on_press_maybe(if self.is_loading {
                            None
                        } else {
                            Some(Message::OpenFile)
                        }),
                    ]
                    .spacing(10),
                    // Keyword
                    row![
                        text_input("Keyword...", &self.keyword).on_input(Message::Kw_change),
                        button("Search").on_press(Message::Search),
                    ]
                    .spacing(10),
                    text(format!(
                        "Matches: {}",
                        count_matches(&self.content, &self.keyword)
                    )),
                    text(&self.error),
                    scrollable(column(lines).spacing(2)).height(Length::Fill),
                    row![
                        Space::with_width(Length::Fill),
                        button("?").on_press(Message::GoTutorial),
                    ],
                ]
                .spacing(10)
                .padding(20)
                .height(Length::Fill);

                if self.confirm_export {
                    page = page.push(
                        column![
                            text("Do you want to export the results?"),
                            row![
                                button("Yes").on_press(Message::ConfirmExport),
                                button("Cancel").on_press(Message::CancelExport),
                            ]
                            .spacing(10)
                        ]
                        .spacing(10),
                    );
                }

                page.into()
            }

            Page::History => {
                let mut page = column![
                    row![
                        text("Search History").size(28),
                        Space::with_width(Length::Fill),
                        button("Back").on_press(Message::GoSearch),
                    ]
                    .align_items(Alignment::Center),
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
                row![
                    text("Tutorial").size(28),
                    Space::with_width(Length::Fill),
                    button("Back").on_press(Message::GoSearch),
                ]
                .align_items(Alignment::Center),
                text("1. Enter your file path."),
                text("2. Click Open to load your file."),
                text("3. Enter a keyword."),
                text("4. Click Search to find matching lines."),
                text("5. Click Load Search History to see your history."),
                text("6. Click Toggle Theme to switch between light and dark mode."),
                text("7. Click Export TXT to save the results to a text file."),
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
