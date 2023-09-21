use std::{env, process};

fn main() {
    let path = env::args().nth(1).unwrap_or(String::from("image.ppm"));

    if let Err(e) = raytracer_rs::run(path) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
