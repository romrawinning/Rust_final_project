mod system;

use iced::widget::{button, column, row, scrollable, text, text_input};
use iced::{Alignment, Length, Element, Sandbox, Settings, Theme, Command};

pub struct Gui_Viewer {
    file_name: String,
    keyword: String,
    content: Vec<String>,
}

pub enum Message {
    File_name_change(String),
    Kw_change(String),
    OpenFile,
    Search,
}
fn main() {
    
}