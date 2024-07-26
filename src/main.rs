mod anki_dictionary;
mod csv_reader;

use anyhow::{Context, Result};
use std::env;

use crate::anki_dictionary::AnkiDictionary;
use crate::csv_reader::read_csv;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_csv> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let entries = read_csv(input_file).context("Failed to read CSV file")?;
    let dictionary = AnkiDictionary::new(entries);
    dictionary
        .save_to_file(output_file)
        .context("Failed to create Anki deck")?;

    println!("Anki deck created successfully: {}", output_file);
    Ok(())
}
