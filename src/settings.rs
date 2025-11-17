use crate::password::{passwords_generator, read_and_store, save_pswd};
use crate::ui::{banner_print, print_help};
use std::error::Error;
use std::process;

pub struct Config {
    length: usize,
    count: usize,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let args: Vec<String> = args.collect();

        if !args.is_empty() {
            match args[0].as_str() {
                "-h" | "--help" => {
                    print_help();
                    process::exit(0);
                }
                "-v" | "--version" => {
                    println!("pswdgen version: {}", env!("CARGO_PKG_VERSION"));
                    process::exit(0);
                }
                _ => {}
            }
        }

        if args.len() < 2 {
            eprintln!("Error missing arguments.\n");
            print_help();
            process::exit(1);
        }
        let length = args[0]
            .parse::<usize>()
            .map_err(|_| "Length must be a number")?;

        let count = args[1]
            .parse::<usize>()
            .map_err(|_| "Count must be a number")?;
        Ok(Config { length, count })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    banner_print();
    let passwords = passwords_generator(config.length, config.count);
    let map = read_and_store(passwords)?;
    save_pswd(&map, "passwords.txt")?;
    println!("Passwords saved to passwords.txt");
    Ok(())
}
