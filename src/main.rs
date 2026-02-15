use std::env::args;

pub mod program; 
pub mod cmd_line;
pub mod data;

fn main()
{
    let args = args().collect();

    program::start_program(args);

    program::keep_app_open_unil_key_pressed();
}