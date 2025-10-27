mod utils;
mod parser;
mod commands;

use std::process;
use std::io::{ self, Write };
use utils::input_loop;

fn main() {
    if let Err(_) = print_welcome_message() {
        process::exit(0);
    }

    input_loop();
}

fn print_welcome_message() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write!(
        handle,
        "
██████╗  ██████╗     ███████╗██╗  ██╗███████╗██╗     ██╗     
██╔══██╗██╔═══██╗    ██╔════╝██║  ██║██╔════╝██║     ██║     
██║  ██║██║   ██║    ███████╗███████║█████╗  ██║     ██║     
██║  ██║██║   ██║    ╚════██║██╔══██║██╔══╝  ██║     ██║     
██████╔╝╚██████╔╝    ███████║██║  ██║███████╗███████╗███████╗
╚═════╝  ╚═════╝     ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝
                                      
"
    )
}
