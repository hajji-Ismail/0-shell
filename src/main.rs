mod utils;
mod parser;
mod commands;
use std::process;
use std::{ io::{ self, Write }};
use utils::{ input_loop };

fn main() {
    print_welcome_message();
    input_loop();
}

fn print_welcome_message() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if
        let Err(_) = writeln!(
            handle,"test"
        )
    {
      process::exit(0);
    }
}
