use std::env;

pub fn cd(path: String) {
    if let Err(e) = env::set_current_dir(path) {
        eprintln!("cd: {}", e);
    }
}
