use caesar::cli::Cli;
use caesar::{encrypt, CaesarError, Key};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let cli = Cli::parse();

    let input_buffer: Box<dyn BufRead>;

    if let Some(file) = cli.file.as_deref() {
        let f = File::open(file).unwrap();
        input_buffer = Box::new(BufReader::new(f));
    } else {
        input_buffer = Box::new(io::stdin().lock());
    }

    run(input_buffer, cli.key).unwrap_or_else(|err| eprintln!("Encryption error: {}", err));
}

pub fn run(buffer: Box<dyn BufRead>, key: i8) -> Result<(), CaesarError> {
    let key = Key::new(key)?;

    for line in buffer.lines() {
        let line = line.unwrap();
        let encrypted = encrypt(&line, &key)?;
        println!("{encrypted}");
    }

    Ok(())
}
