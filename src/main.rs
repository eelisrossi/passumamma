use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use std::{env, error::Error, ffi::OsString, fs::File, process};

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    let mut words: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let word = &record[0];
        words.push(word.to_string());
    }

    for n in 0..10 {
        println!("{}: {}", n, create_passphrase(&words));
    }

    Ok(())
}

// TODO: add check for word length so only words 3-5 char long will be used
fn get_random_word(words: &Vec<String>) -> String {
    let mut rng = ChaCha20Rng::from_entropy();
    let index = rng.gen_range(0..words.len());
    let random_word = &words[index];
    random_word.to_string()
}

// TODO: add uppercase + numbers
fn create_passphrase(words: &Vec<String>) -> String {
    let mut rng = ChaCha20Rng::from_entropy();
    let number = rng.gen_range(100..999);
    let mut first_word = get_random_word(&words);
    let mut second_word = get_random_word(&words);
    let random_capitalize = rng.gen_bool(0.5 / 1.0);
    if random_capitalize {
        first_word = first_word.to_uppercase();
    } else {
        second_word = second_word.to_uppercase();
    }
    let passphrase = format!("{}-{}{}", first_word, second_word, number);
    passphrase
}
