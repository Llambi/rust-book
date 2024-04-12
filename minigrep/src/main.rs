use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Probleme parsing arguments:\n{err}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:\n{e}");
        process::exit(1);
    }
}
