use vergen::{vergen, Config};

fn main() {
    let r = vergen(Config::default());
    match r {
        Err(e) => panic!("vergen: {}", e),
        _ => {}
    }
}
