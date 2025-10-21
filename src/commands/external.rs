use nix::unistd::{fork, ForkResult, execvp};
use nix::sys::wait::wait;
use std::ffi::CString;
use std::env;
use crate::utils::Parsing;

/// Run any external command using the Parsing struct
pub fn run_external_command(parsed: &Parsing) {
    println!("parsed{:?}", parsed);
    // Ensure /usr/sbin is in PATH
    let path = env::var("PATH").unwrap_or_default();
    if !path.contains("/usr/sbin") {
        let new_path = format!("{}:/usr/sbin", path);
        unsafe {
            env::set_var("PATH", new_path); // nightly requires unsafe
        }
    }

    // Build args vector: first element = command, then flags, then arguments
    let mut args: Vec<CString> = Vec::with_capacity(1 + parsed.flag.len() + parsed.arg.len());
    args.push(CString::new(parsed.command.clone()).unwrap());

    for flag in &parsed.flag {
        args.push(CString::new(flag.as_str()).unwrap());
    }
    for arg in &parsed.arg {
        args.push(CString::new(arg.as_str()).unwrap());
    }

    // Fork child
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            let _ = wait();
        }
        Ok(ForkResult::Child) => {
            // Execute command; on error print and exit
            let _ = execvp(&CString::new(parsed.command.clone()).unwrap(), &args)
                .unwrap_or_else(|_| {
                    eprintln!("{}: command not found", parsed.command);
                    std::process::exit(1);
                });
        }
        Err(e) => eprintln!("Fork failed: {}", e),
    }
}
