use std::fs;
use std::io::{self, BufRead};

pub fn read_file_lines(filename: String) {
    let result = fs::read_to_string(filename).expect("reading file");
    println!("{}", result);
}

// reads from standard input and copy to standard output
pub fn echo_mode() -> Result<std::string::String, std::io::Error> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(e) => return Err(e),
            Ok(l) => {
                if l == "EOF" {
                    return Ok(String::from("finished"));
                } else {
                    println!("{}", l);
                }
            }
        }
    }
    Ok(String::from("finished"))
}

// TODO: handle from env input, replace codes in main function
pub fn handle_args() {}

mod test {
    use crate::read::read_file_lines;

    #[test]
    fn test_read_example_file() {
        read_file_lines(String::from("example.txt"))
    }
}
