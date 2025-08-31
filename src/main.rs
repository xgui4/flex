use std::io;
use std::env;

pub mod program; 

fn main()
{

    let args: Vec<_> = env::args().collect();

    if args[1] == "script-mode" {
        program::start_program();
    }
    else {
        program::start_program();
        println!("Press any key to quit...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}