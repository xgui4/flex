use crate::data::app_data;
use crate::data::colors;

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
            Err(e) => println!("{ERROR_FILE_CANNOT_BE_READ}: {}", e),
        },
        Err(_) => println!("{ERROR_FILE_NOT_FOUND}"),
    }
}

fn print_help() {
    for string in &data::HELP_STRING {
        println!(string); 
    }
}

fn print_colors_code_references() {
    for string in &data::COLOR_CODE_HELP_STRING {
        println!(string);
    }
}