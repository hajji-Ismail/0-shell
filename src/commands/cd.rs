use std::env;
use std::path::Path;

use crate::utils::Parsing;

pub fn cd(input: Parsing) {
    if !input.flag.is_empty(){
        println!("bash: cd: {}: invalid option", input.flag[0]);
        return
    }
    if input.arg.len() > 1 {
        println!("bash: cd: too many arguments");
        return
    }
    // Determine the target path
    let target = if let Some(first_arg) = input.arg.get(0) {
        let p = first_arg.trim();


        // Handle "cd -" (go to previous directory)
        if p == "-" {
            match env::var("OLDPWD") {
                Ok(oldpwd) => Path::new(&oldpwd).to_path_buf(),
                Err(_) => {
                    eprintln!("cd: OLDPWD not set");
                    return;
                }
            }
        }
        // Handle "~" or "~/something"
        else if p.starts_with('~') {
            if let Ok(home) = env::var("HOME") {
                let expanded = p.replacen("~", &home, 1);
                Path::new(&expanded).to_path_buf()
            } else {
                Path::new("/").to_path_buf()
            }
        }
        // Regular path
        else {
            Path::new(p).to_path_buf()
        }
    } else {
        // No arguments → go to $HOME or fallback to /
        match env::var("HOME") {
            Ok(home) => Path::new(&home).to_path_buf(),
            Err(_) => Path::new("/").to_path_buf(),
        }
    };

    // Store current directory as OLDPWD before changing
    if let Ok(current) = env::current_dir() {
        unsafe { env::set_var("OLDPWD", current);
    }
}
       

    // Try to change directory
    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}", e);
    }
}
