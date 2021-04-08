mod read;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: write test files!
    if args.len() == 1 {
        println!("running in echo mode!");
        read::echo_mode().expect("echo mode error");
        return;
    }

    let filename = &args[1];

    if filename == "-" {
        println!("running in echo mode!");
        read::echo_mode().expect("echo mode error");
        return;
    }

    read::read_file_lines(filename.to_string());
}
