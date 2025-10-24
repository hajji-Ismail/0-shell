use crate::Parsing;
use std::fs::File;
use std::io::{self, Read, Write};
use std::fs;
use std::os::unix::fs::FileTypeExt;

pub fn cat(input: Parsing) {
    // Handle invalid flags
    if !input.flag.is_empty() {
        eprintln!("cat: unrecognized option '{}'", input.flag[0]);
        return;
    }

    // No arguments => read from stdin
    if input.arg.is_empty() {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut buffer = [0u8; 1024];

        loop {
            match stdin.lock().read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    if stdout.write_all(&buffer[..n]).is_err() {
                        break;
                    }
                    stdout.flush().ok();
                }
                Err(e) => {
                    eprintln!("cat: error reading stdin: {}", e);
                    break;
                }
            }
        }
        return;
    }

    // Process each file argument
    for path in input.arg {
        match File::open(&path) {
            Ok(mut file) => {
                // Check metadata (handle device or special files)
                let is_device = match fs::metadata(&path) {
                    Ok(meta) => {
                        let ft = meta.file_type();
                        ft.is_char_device() || ft.is_block_device()
                    }
                    Err(_) => false,
                };

                let mut stdout = io::stdout();
                let mut buffer = [0u8; 8192];
                let mut total_read = 0usize;
                const MAX_DEVICE_BYTES: usize = 1024 * 1024; // Limit 1MB for devices

                loop {
                    match file.read(&mut buffer) {
                        Ok(0) => break, // EOF
                        Ok(n) => {
                            if stdout.write_all(&buffer[..n]).is_err() {
                                break;
                            }

                            // Prevent infinite reads on endless device files
                            if is_device {
                                total_read += n;
                                if total_read >= MAX_DEVICE_BYTES {
                                    eprintln!("cat: {}: device file truncated at 1MB to prevent infinite read", path);
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("cat: {}: {}", path, e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("cat: {}: {}", path, e);
            }
        }
    }
}
