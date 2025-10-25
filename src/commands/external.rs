use nix::unistd::{ fork, ForkResult, execvp };
use nix::sys::wait::wait;
use std::ffi::CString;
use std::env;
use crate::parser::Parsing;

pub fn run_external_command(parsed: &Parsing) {
    let path = env::var("PATH").unwrap_or_default();
    if !path.contains("/usr/sbin") {
        let new_path = format!("{}:/usr/sbin", path);
        unsafe {
            env::set_var("PATH", new_path);
        }
    }

    let mut args: Vec<CString> = Vec::with_capacity(1 + parsed.flags.len() + parsed.args.len());
    args.push(CString::new(parsed.command.clone()).unwrap());

    for flag in &parsed.flags {
        args.push(CString::new(flag.as_str()).unwrap());
    }
    for arg in &parsed.args {
        args.push(CString::new(arg.as_str()).unwrap());
    }

    // Fork child
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            let _ = wait();
        }
        Ok(ForkResult::Child) => {
            // Execute command; on error print and exit
            if let Err(_) = execvp(&CString::new(parsed.command.clone()).unwrap(), &args) {
                eprintln!("{}: command not found", parsed.command);
                std::process::exit(1);
            }
        }
        Err(e) => eprintln!("Fork failed: {}", e),
    }
}
