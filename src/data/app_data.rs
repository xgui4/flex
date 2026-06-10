pub const VERSION : &str = "0.0.20"; 
pub const AUTHOR : &str = "Xgui4"; 
pub const LICENSE : &str = "GPL3";
pub const APP_NAME : &str = "Flex-rs";
pub const YEAR: &str = "2026";
pub const LICENSE_STR: &str = "Copyleft ©️ {YEAR} {AUTHOR} | {LICENSE} License";

pub const PATH_INPUT_LABEL_KEY : &str = "enter_path_label"; 
pub const ABOUT_LABEL_KEY: &str = "about_label_key";
pub const OPTION_LABEL_KEY: &str = "option_label";
pub const PATH_INPUT_HELP_LABEL_KEY: &str = "path_input_help_label";
pub const PRESS_ANY_KEY_TO_QUIT_KEY: &str = "press_any_key_to_quit";
pub const COLOR_CODE_REFERENCE_KEY: &str = "color_code_reference";

pub const ERROR_READ_CONSOLE_KEY : &str = "error_read_console"; 
pub const ERROR_FILE_CANNOT_BE_READ_KEY: &str = "error_file_unreable";
pub const ERROR_FILE_NOT_FOUND_KEY: &str = "error_file_not_found";

pub const HELP_STRING: [&str; 6]  = [
    OPTION_LABEL_KEY,
    "--License or --l",
    "--about or --a",
    "--version or --l",
    "--color-code or --c",
    PATH_INPUT_HELP_LABEL_KEY
]; 

pub const COLOR_CODE_HELP_STRING: [&str; 8]  = [
    COLOR_CODE_REFERENCE_KEY, 
    "color1",
    "color2",
    "color3",
    "color4",
    "color5",
    "color6",
    "color0",
]; 

pub const LICENSE_TEXT_STR: &str = r#""
    Flex-rs is a simple Rust project designed to display ASCII art from `.ascii` files.
    Copyright (C) 2026 Xgui4

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
"#;  



