mod read;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: read from standard input with no FILE input or "-"
    if args.len() <= 1 {
        panic!("please specify a file");
    }

    let filename = &args[1];

    read::read_file_lines(filename.to_string());
}
