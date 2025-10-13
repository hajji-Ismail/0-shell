use std::env;
use std::path::Path;

pub fn cd(path: Option<&str>) {
    let target = match path {
        Some(p) => Path::new(p).to_path_buf(),
        None => Path::new("~").to_path_buf()
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}", e);
    }
}
