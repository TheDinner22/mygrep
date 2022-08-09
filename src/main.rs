use std::{process, env};

use mygrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    if let Err(e) = mygrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    };
}

// im at start of 12.5