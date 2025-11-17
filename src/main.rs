mod constants;
mod password;
mod settings;
mod ui;

use settings::{Config, run};
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
