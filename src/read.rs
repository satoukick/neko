use std::fs;
use std::io::{self, BufRead};

pub fn read_file_lines(filename: String) {
    let result = fs::read_to_string(filename).expect("error reading file");
    println!("{}", result);
}

pub fn echo_mode() -> Result<std::string::String, std::io::Error> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(e) => return Err(e),
            Ok(l) => println!("{}", l),
        }
    }
    Ok(String::from("finished"))
}

// TODO: handle from env input, replace codes in main function
pub fn handle_args() {}
