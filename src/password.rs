use rand::{Rng, rngs::OsRng};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, Write},
};

use crate::constants::SYMBOLS;

pub fn passwords_generator(length: usize, count: usize) -> Vec<String> {
    let mut rnd_range = OsRng;
    let pswd = (0..count)
        .map(|_| {
            (0..length)
                .map(|_| {
                    let idx = rnd_range.gen_range(0..SYMBOLS.len());
                    SYMBOLS[idx] as char
                })
                .collect()
        })
        .collect();
    pswd
}

pub fn read_and_store(passwords: Vec<String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut hash_map = HashMap::new();
    let mut input = String::new();
    for (ind, pswd) in passwords.iter().enumerate() {
        print!("Would you like to specify the passwords name?\nAnswers: 'yes' or 'no'\n");
        input.clear();
        io::stdin().read_line(&mut input)?;
        let answer = input.trim().to_lowercase();
        if answer == "y" || answer == "yes" {
            print!("Enter the name of resource or website: \n");
            input.clear();
            io::stdin().read_line(&mut input)?;
            let name = input.trim().to_string();
            hash_map.insert(name, pswd.clone());
        } else {
            let ind_name = format!("password #{}", ind + 1);
            hash_map.insert(ind_name, pswd.clone());
        }
    }
    Ok(hash_map)
}

pub fn save_pswd(data: &HashMap<String, String>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    for (key, val) in data {
        writeln!(file, "{}: {}", key, val)?;
    }
    Ok(())
}
