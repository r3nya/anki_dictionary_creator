mod anki_dictionary;
mod csv_reader;
mod error;

use std::env;

use crate::anki_dictionary::AnkiDictionary;
use crate::csv_reader::read_csv;
use crate::error::AnkiDictionaryError;

fn main() -> Result<(), AnkiDictionaryError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_csv> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let entries = read_csv(input_file)?;
    let dictionary = AnkiDictionary::new(entries);
    dictionary.save_to_file(output_file)?;

    println!("Anki deck created successfully: {}", output_file);
    Ok(())
}
