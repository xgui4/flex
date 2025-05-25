use std::fs;
use std::io;
use std::env;

const PATH_INPUT_LABEL: &str = "Enter the full path or relative path of the ASCII art file you want to display (without the .ascii extension):";
const ERROR_READ_CONSOLE: &str = "Error : failed to read console"; 
const ERROR_FILE_NOT_FOUND: &str = "Should have been able to read the file"; 

pub fn start_program() {
    let args: Vec<_> = env::args().collect(); 
    if args.len() > 1 {
        if args[1] == "--version" || args[1] == "--v" {
            println!("Flex Version 0.0.1 Developper Preview 1");
        }
        if args[1] == "--license" || args[1] == "--l" {
            println!("Copyright ©️ 2025 Xgui4 Studio | MIT License");
        }
        if args[1] == "--about" || args[1] == "--a" {
            println!("Flex is a simple Rust project designed to display ASCII art from .ascii files.")
        }
        if args[1] == "--help" || args[1] == "--h" {
            println!("flex [option]"); 
            println!("flex <path to the .ascii file> (without the .ascii extention"); 
        }
        else {
            output_image(&args[1]);
        }
    }
    else {
        choice_user();
    }
}

fn choice_user() {
    println!("{PATH_INPUT_LABEL}");
    let mut user_choice = String::new(); 
    io::stdin().read_line(&mut user_choice).expect(ERROR_READ_CONSOLE);

    output_image(&user_choice);
}

fn output_image(path: &str) {
    let user_choice = path.trim();
    let file_path = format!("{}.ascii", user_choice); 

    let image = fs::read_to_string(&file_path).expect(ERROR_FILE_NOT_FOUND); 

    println!("{image}"); 
}