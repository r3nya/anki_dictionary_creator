use anyhow::{Context, Result};
use csv::Reader;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct QnAEntry {
    pub question: String,
    pub answer: String,
}

pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<QnAEntry>> {
    let file = File::open(path).context("Failed to open CSV file")?;
    let mut reader = Reader::from_reader(file);
    let entries: Result<Vec<QnAEntry>, _> = reader
        .deserialize()
        .collect::<Result<Vec<_>, csv::Error>>()
        .context("Failed to parse CSV entries");
    entries
}
