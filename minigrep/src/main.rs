use minigrep::Config;
use std::{env, process};

fn main() {
    let args = env::args().skip(1).into_iter();

    let config = Config::new(args).unwrap_or_else(|errs| {
        for err in errs {
            println!("{}", err)
        }
        process::exit(1);
    });

    println!("{:?}", config.query);
    println!("{:?}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("error {}", e);
        process::exit(1);
    }
}
