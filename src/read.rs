use std::fs;
use std::io::{self, BufRead};

use crate::options::NekoOptions;
use crate::utils;

pub fn read_file_lines(filename: &String) {
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
                println!("{}", l);
            }
        }
    }
    Ok(String::from("finished"))
}

// TODO: premature idea: convert arguments to vector of enum Options,
// in order to know all requested options and files before starting to output
pub fn decide_modes(args: &[String]) -> Result<Vec<NekoOptions>, std::io::Error> {
    let mut modes: Vec<NekoOptions> = Vec::new();

    for arg in args {
        let processed_arg = utils::remove_whitespaces(arg.as_str());
        let mode = match processed_arg.as_str() {
            "-n" => NekoOptions::ShowEnds,
            "--version" => NekoOptions::Version,
            "-" => NekoOptions::StdInput,

            _ => NekoOptions::StdInput,
        };
        modes.push(mode);
    }

    Ok(modes)
}

pub fn handle_args(args: &[String]) -> Result<std::string::String, std::io::Error> {
    for arg in args {
        // echo mode
        if arg == "-" {
            echo_mode().expect("running echo mode");
            continue;
        }

        // if arg[-]

        let filename = arg;
        read_file_lines(filename);
    }

    Ok(String::from("handle arguments successful"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_example_file() {
        read_file_lines(&String::from("example.txt"))
    }

    #[test]
    fn test_handle_args() {
        let args = &[String::from("example.txt")];
        let result = handle_args(args);
        assert!(result.is_ok())
    }
}
