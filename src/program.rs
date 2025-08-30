use std::{fs, io};
use std::path::Path;
use std::{env};

const PATH_INPUT_LABEL: &str = "Enter the full path or relative path of the ASCII art file you want to display:";
const ERROR_READ_CONSOLE: &str = "Error : failed to read console";
const ERROR_FILE_CANNOT_BE_READ: &str = "Error : Should have been able to read the file";
const ERROR_FILE_NOT_FOUND: &str = "Error : file cannot be read";
const VERSION: &str = "0.0.1 Developper Preview 4";
const LICENSE: &str = "Copyright ©️ 2025 Xgui4 Studio | MIT License";
const ABOUT: &str = "Flex is a simple Rust project designed to display ASCII art from .ascii files.";
const OPTION_LABEL: &str = "[option]";
const PATH_INPUT_HELP_LABEL: &str = "<path to the .ascii file>";

pub fn start_program() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "--version" || args[1] == "--v" {
            println!("Flex Version {VERSION}");
        }
        if args[1] == "--license" || args[1] == "--l" {
            println!("{LICENSE}");
        }
        if args[1] == "--about" || args[1] == "--a" {
            println!("{ABOUT}")
        }
        if args[1] == "--help" || args[1] == "--h" {
            println!("flex {OPTION_LABEL}");
            println!("flex {PATH_INPUT_HELP_LABEL}");
        }
        if args[1] == "--create" || args[1] == "--c" {
            println!("Create mode is not yet implemented. Create your own .ascii files manually for now.");
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
            Ok(image) => println!("{image}"),
            Err(e) => println!("{ERROR_FILE_CANNOT_BE_READ}: {}", e),
        },
        Err(_) => println!("{ERROR_FILE_NOT_FOUND}"),
    }
}

fn create_window() {
    println!("GUI mode is not yet implemented. Please use the command line interface.");
}