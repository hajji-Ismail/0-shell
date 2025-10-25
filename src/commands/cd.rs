use std::env;
use std::path:: PathBuf;

use crate::utils::Parsing;

pub fn cd(input: Parsing) {
    if !input.flag.is_empty() {
        println!("bash: cd: {}: invalid option", input.flag[0]);
        return;
    }
    if input.arg.len() > 2 {
        println!("bash: cd: too many arguments");
        return;
    }

    // Handle the 2-argument substitution mode
    let  target = if input.arg.len() == 2 {
        // Get current PWD as string
        let pwd = match env::var("PWD") {
            Ok(p) => p,
            Err(_) => {
                eprintln!("cd: PWD not set");
                return;
            }
        };

        let pattern = input.arg[0].trim();
        let replacement = input.arg[1].trim();

        // Check if pattern is in PWD
        if !pwd.contains(pattern) {
            eprintln!("cd: string not in pwd: {}", pattern);
            return;
        }

        // Replace first occurrence
        let new_path = pwd.replacen(pattern, replacement, 1);
        PathBuf::from(new_path)
    } else {
        // Determine the target path for 0 or 1 arg
        if let Some(first_arg) = input.arg.get(0) {
            let p = first_arg.trim();

            // Handle "cd -" (go to previous directory)
            if p == "-" {
                match env::var("OLDPWD") {
                    Ok(oldpwd) => PathBuf::from(oldpwd),
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
                    PathBuf::from(expanded)
                } else {
                    PathBuf::from("/")
                }
            }
            // Regular path
            else {
                PathBuf::from(p)
            }
        } else {
            // No arguments → go to $HOME or fallback to /
            match env::var("HOME") {
                Ok(home) => PathBuf::from(home),
                Err(_) => PathBuf::from("/"),
            }
        }
    };

    // Store current directory as OLDPWD before changing
    if let Ok(current) = env::current_dir() {
        unsafe {
            env::set_var("OLDPWD", current);
            
        }
    }

    // Try to change directory
    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target.display(), e);
        return;
    }

    // Update PWD after successful change
    if let Ok(new_pwd) = env::current_dir() {
        unsafe {env::set_var("PWD", new_pwd);}
        
    }

    // For "cd -", print the new directory
    if input.arg.get(0).map_or(false, |arg| arg.trim() == "-") {
        println!("{}", target.display());
    }
}