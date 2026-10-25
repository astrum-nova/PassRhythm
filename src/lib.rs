use std::fs::{File, OpenOptions};

/// General file opening function
pub fn open_file(file_path: &str, truncate: bool) -> File {
    if let Ok(file) = OpenOptions::new().read(true).write(true).create(true).truncate(truncate).open(file_path) {
        file
    } else {
        panic!("Failed to open or create {file_path}")
    }
}
