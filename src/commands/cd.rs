use std::env;
use std::path::Path;

pub fn cd(path: Option<&str>) {
    // If no path provided, try to go to $HOME
    let target = match path {
        Some(p) => Path::new(p).to_path_buf(),
        None => match env::var("HOME") {
            Ok(home) => Path::new(&home).to_path_buf(),
            Err(_) => {
                eprintln!("cd: HOME not set");
                return;
            }
        },
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: cannot change directory to '{}': {}", target.display(), e);
    }
}
