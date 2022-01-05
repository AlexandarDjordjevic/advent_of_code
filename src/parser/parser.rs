use core::panic;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[allow(unused)]
pub fn parse_input_file(file: &Path) -> String {
    let display = file.display();

    let mut file = match File::open(&file) {
        Err(reason) => panic!("couldn't open {}: {}", display, reason),
        Ok(file) => file,
    };
    let mut commands = String::new();
    match file.read_to_string(&mut commands) {
        Ok(_) => return commands,
        Err(reason) => panic!("Error reading file: {}", reason),
    }
}
