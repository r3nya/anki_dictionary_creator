use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnkiDictionaryError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Genanki error: {0}")]
    Genanki(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Failed to create note: {0}")]
    NoteCreation(String),
}
