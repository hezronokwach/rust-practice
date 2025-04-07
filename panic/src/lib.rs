
use std::fs::File;
use std::io:: ErrorKind;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                panic!("File not found: {}", s);
            } else {
                panic!("Error opening file {}: {}", s, error);
            }
        }
    }
}
