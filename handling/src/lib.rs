use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file_result = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path);
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                // File doesn't exist, create it
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(path)
                {
                    Ok(file) => file,
                    Err(e) => panic!("Failed to create the file: {:?}", e),
                }
            } else {
                // Some other error occurred
                panic!("Problem opening the file: {:?}", error);
            }
        }
    };
    
    // Write the content to the file
    if let Err(e) = file.write_all(content.as_bytes()) {
        panic!("Failed to write to the file: {:?}", e);
    }
}