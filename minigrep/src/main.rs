use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("err: {}", err);
        process::exit(1);
    });

    println!("{:?}", config.query);
    println!("{:?}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("error {}", e);
        process::exit(1);
    }
}

