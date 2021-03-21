use std::fs;

pub fn read_file_lines(filename: String) {
    let result = fs::read_to_string(filename).expect("error reading file");
    println!("{}", result);
}
