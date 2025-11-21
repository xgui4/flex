use std::{fs, io};
use std::path::Path;

const PATH_INPUT_LABEL: &str = "Enter the full path or relative path of the ASCII art file you want to display:";
const ERROR_READ_CONSOLE: &str = "Error : failed to read console";
const ERROR_FILE_CANNOT_BE_READ: &str = "Error : Should have been able to read the file";
const ERROR_FILE_NOT_FOUND: &str = "Error : file cannot be read";
const VERSION: &str = "0.0.1 Developper Preview 4";
const LICENSE: &str = "Copyright ©️ 2025 Xgui4 Studio | MIT License";
const ABOUT: &str = "Flex-rs is a simple Rust project designed to display ASCII art from .ascii files.";
const OPTION_LABEL: &str = "[option]";
const PATH_INPUT_HELP_LABEL: &str = "<path to the .ascii file>";


pub fn start_script_mode() {
    println!("Feature Not Implemented Yet! Try using normal mode for now")
}

pub fn start_program(args: Vec<String>) {
    if args.len() > 1 {
        if args[1] == "--version" || args[1] == "--v" {
            println!("Flex-rs Version {VERSION}");
        }
        if args[1] == "--license" || args[1] == "--l" {
            println!("{LICENSE}");
        }
        if args[1] == "--about" || args[1] == "--a" {
            println!("{ABOUT}")
        }
        if args[1] == "--help" || args[1] == "--h" {
            println!("flex-rs {OPTION_LABEL}");
            println!("flex-rs --License or --l"); 
            println!("flex-rs --about or --a");
            println!("flex-rs --version or --l"); 
            println!("flex-rs --create or --c (coming soon)");
            println!("flex-rs --gui or --g (coming soon!)"); 
            println!("flex-rs --color-code or --cc");
            println!("flex-rs {PATH_INPUT_HELP_LABEL}");
        }
        if args[1] == "--create" || args[1] == "--c" {
            println!("Features Not Implemented Yet !")
        }
        if args[1] == "--color-code" || args[1] == "--cc" {
            println!("Color Code Reference:");
            println!("$1 : Red");
            println!("$2 : Green");
            println!("$3 : Yellow");
            println!("$4 : Blue");
            println!("$5 : Magenta");
            println!("$6 : Cyan");
            println!("$0 : Reset");
        }
        if args[1] == "--gui" || args[1] == "--g" {
            create_window();
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
    let file_path = format!("{}", user_choice);

    let path = Path::new(&file_path);
    let absolute_path = fs::canonicalize(path);

    match absolute_path {
        Ok(real_path) => match fs::read_to_string(real_path) {
            Ok(image) => {
            // Simple color replacement: $1, $2, ... replaced by ANSI color codes
            let colored_image = image
                .replace("$1", "\x1b[31m") // Red
                .replace("$2", "\x1b[32m") // Green
                .replace("$3", "\x1b[33m") // Yellow
                .replace("$4", "\x1b[34m") // Blue
                .replace("$5", "\x1b[35m") // Magenta
                .replace("$6", "\x1b[36m") // Cyan
                .replace("$0", "\x1b[0m"); // Reset
            println!("{colored_image}\x1b[0m");
            },
            Err(e) => println!("{ERROR_FILE_CANNOT_BE_READ}: {}", e),
        },
        Err(_) => println!("{ERROR_FILE_NOT_FOUND}"),
    }
}

fn create_window() {
    println!("GUI mode is not yet implemented. Please use the command line interface.");
}