use std::env;
use std::fs; 
use std::io;

fn main()
{
    println!("Enter the full path or relative path of the ASCII art file you want to display (without the .ascii extension):");

    let mut user_choice = String::new(); 
    io::stdin().read_line(&mut user_choice).expect("Error : failed to read console");

    let user_choice = user_choice.trim();
    let file_path = format!("{}.ascii", user_choice); 

    let image = fs::read_to_string(&file_path).expect("Should have been able to read the file"); 

    println!("{image}"); 
}