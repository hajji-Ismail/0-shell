use std::env;
use std::path::PathBuf;
use std::fs;

use crate::parser::Parsing;

pub fn cd(input: Parsing) {
    if !input.flags.is_empty() {
        println!("bash: cd: {}: invalid option", input.flags[0]);
        return;
    }
    if input.args.len() > 2 {
        println!("bash: cd: too many arguments");
        return;
    }

    let target = if input.args.len() == 2 {
        let pwd = match env::var("PWD") {
            Ok(p) => p,
            Err(_) => {
                eprintln!("cd: PWD not set");
                return;
            }
        };

        let pattern = input.args[0].trim();
        let replacement = input.args[1].trim();

        // Check if pattern is in PWD
        if !pwd.contains(pattern) {
            eprintln!("cd: string not in pwd: {}", pattern);
            return;
        }

        // Replace first occurrence
        let new_path = pwd.replacen(pattern, replacement, 1);
        PathBuf::from(new_path)
    } else {
        if let Some(first_arg) = input.args.get(0) {
            let p = first_arg.trim();

            if p == "-" {
                match env::var("OLDPWD") {
                    Ok(oldpwd) => PathBuf::from(oldpwd),
                    Err(_) => {
                        eprintln!("cd: OLDPWD not set");
                        return;
                    }
                }
            } else {
                PathBuf::from(p)
            }
        } else {
            match env::var("HOME") {
                Ok(home) => PathBuf::from(home),
                Err(_) => PathBuf::from("/"),
            }
        }
    };

    if let Ok(current) = env::current_dir() {
        unsafe {
            env::set_var("OLDPWD", current);
        }
    }

    // Compute absolute path for symlink case
    let absolute_target = if target.is_absolute() {
        target.clone()
    } else {
        match env::current_dir() {
            Ok(current) => current.join(&target),
            Err(_) => {
                eprintln!("cd: cannot access current directory");
                return;
            }
        }
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target.display(), e);
        return;
    }

    if
        fs
            ::symlink_metadata(&target)
            .map(|m| m.is_symlink())
            .unwrap_or(false)
    {
        unsafe {
            env::set_var("PWD", &absolute_target);
        }
    } else {
        if let Ok(new_pwd) = env::current_dir() {
            unsafe {
                env::set_var("PWD", new_pwd);
            }
        }
    }

    if input.args.get(0).map_or(false, |arg| arg.trim() == "-") {
        println!("{}", target.display());
    }
}
