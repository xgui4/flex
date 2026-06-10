use crate::data::colors::{self, Colors};

pub fn colorized_text(txt : &str) -> String {
    let colored_text = txt
    .replace(Colors::Blue.get_color_code(), colors::get_escape_code(Colors::Blue))
    .replace(Colors::Cyan.get_color_code(), colors::get_escape_code(Colors::Cyan))
    .replace(Colors::Green.get_color_code(), colors::get_escape_code(Colors::Green))
    .replace(Colors::Red.get_color_code(), colors::get_escape_code(Colors::Red))
    .replace(Colors::Magenta.get_color_code(), colors::get_escape_code(Colors::Magenta))
    .replace(Colors::Yellow.get_color_code(), colors::get_escape_code(Colors::Yellow))
    .replace(Colors::Reset.get_color_code(), colors::get_escape_code(Colors::Reset)); 
    return colored_text; 
}