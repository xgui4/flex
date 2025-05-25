pub mod program; 
use std::io; 

fn main()
{
    program::start_program();

    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}