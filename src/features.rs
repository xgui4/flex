use std::fs;
use std::io;
use std::path::Path;

use crate::data::app_data;
use crate::data::colors;

const APP_NAME: &str = app_data::APP_NAME; 
const VERSION: &str = app_data::VERSION; 
const LICENSE: &str = app_data::LICENSE;

pub fn display_app_version_info() {
    println!("{APP_NAME} Version {VERSION}");
}

pub fn display_license_info() {
    println!("{LICENSE}");
}

pub fn display_about_info() {
    let about_str : &str = &t!(app_data::ABOUT_LABEL_KEY);
    println!("{about_str}");
}

pub fn keep_app_open_unil_key_pressed() {
    let press_any_key_to_quit_str : &str = &t!(app_data::PRESS_ANY_KEY_TO_QUIT_KEY);
    println!("{press_any_key_to_quit_str}");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

pub fn choice_user() {
    let path_input_label_str : &str = &t!(app_data::PATH_INPUT_HELP_LABEL_KEY);
    let error_read_console_str : &str = &t!(app_data::ERROR_READ_CONSOLE_KEY);
    println!("{path_input_label_str}");
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect(error_read_console_str);

    output_image(&user_choice);
}

pub fn output_image(path: &str) {
    let user_choice = path.trim();
    let file_path = format!("{}", user_choice);

    let path = Path::new(&file_path);
    let absolute_path = fs::canonicalize(path);

    let error_file_cannot_be_read: &str = &t!(app_data::ERROR_FILE_CANNOT_BE_READ_KEY);
    let error_file_not_found:&str = &t!(app_data::ERROR_FILE_NOT_FOUND_KEY);

    match absolute_path {
        Ok(real_path) => match fs::read_to_string(real_path) {
            Ok(image) => {
            let colored_image = image
                .replace("$1", colors::get_color_color_code_fom_code("$1"))
                .replace("$2", colors::get_color_color_code_fom_code("$2"))
                .replace("$3", colors::get_color_color_code_fom_code("$3")) 
                .replace("$4", colors::get_color_color_code_fom_code("$4")) 
                .replace("$5", colors::get_color_color_code_fom_code("$5"))
                .replace("$6", colors::get_color_color_code_fom_code("$6"))
                .replace("$0", colors::get_color_color_code_fom_code("$7"));
            println!("{colored_image}\x1b[0m");
            },
            Err(e) => println!("{error_file_cannot_be_read}: {}", e),
        },
        Err(_) => println!("{error_file_not_found}"),
    }
}

pub fn print_help() {
    for string in &app_data::HELP_STRING {
        println!("{string}"); 
    }
}

pub fn print_colors_code_references() {
    for string in &app_data::COLOR_CODE_HELP_STRING {
        println!("{string}");
    }
}