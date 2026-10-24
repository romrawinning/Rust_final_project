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
fn main() {
    


}