use std::fs::File;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    let mut file = match File::create(file) {
        Ok(file) => file,
        Err(e) => panic!("Error creating or opening the file: {}", e),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("Error writing to the file: {}", e),
    }
}
