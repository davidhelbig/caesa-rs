use std::path::PathBuf;

use clap::Parser;

/// A simple cli for "encrypting" ascii text using a shift cipher, also
/// known as Caesar cipher.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Shift by how many characters in the alphabet.
    #[arg(short, long, default_value_t = 3)]
    pub key: i8,

    /// File to process. If not indicated, input is read from stdin.
    pub file: Option<PathBuf>,
}
