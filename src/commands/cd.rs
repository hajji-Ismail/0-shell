use std::env;
use std::path::Path;

pub fn cd(path: Option<&str>) {
    let target = match path {
        Some(p) if !p.trim().is_empty() => Path::new(p).to_path_buf(),
        _ => match env::var("HOME") {
            Ok(home) => Path::new(&home).to_path_buf(),
            Err(_) => Path::new("/").to_path_buf(), 
        },
    };



    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}", e);
    }
}
