mod read;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let files = &args[1..];

    let result = read::handle_args(files);

    match result {
        Err(err) => {
            println!("neko did not succeed: {}", err);
            process::exit(1);
        }
        Ok(_) => {
            println!("neko finished.");
        }
    }
}
