use crate::error::AnkiDictionaryError;
use csv::Reader;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct QnAEntry {
    pub question: String,
    pub answer: String,
}

pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<QnAEntry>, AnkiDictionaryError> {
    let file = File::open(path).map_err(AnkiDictionaryError::Io)?;
    let mut reader = Reader::from_reader(file);
    let entries: Result<Vec<QnAEntry>, csv::Error> = reader.deserialize().collect();
    entries.map_err(AnkiDictionaryError::Csv)
}
