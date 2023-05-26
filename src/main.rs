use std::{env, error::Error, ffi::OsString, fs::File, process};

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = get_words() {
        println!("{}", err);
        process::exit(1);
    }
}

fn get_words() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    let mut words: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let word = &record[0];
        words.push(word.to_string());
    }

    println!("{:?}", words);

    Ok(())
}
