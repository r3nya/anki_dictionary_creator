use crate::csv_reader::QnAEntry;
use crate::error::AnkiDictionaryError;
use genanki_rs::{Deck, Field, Model, Note, Template};
use std::path::Path;

pub struct AnkiDictionary {
    entries: Vec<QnAEntry>,
}

impl AnkiDictionary {
    pub fn new(entries: Vec<QnAEntry>) -> Self {
        Self { entries }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), AnkiDictionaryError> {
        let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n}\n";
        let model = Model::new(
            1607392319,
            "Simple Model",
            vec![Field::new("Question"), Field::new("Answer")],
            vec![Template::new("Card 1")
                .qfmt("{{Question}}")
                .afmt("{{FrontSide}}<hr id=\"answer\">{{Answer}}")],
        )
        .css(custom_css);

        let mut deck = Deck::new(2059400110, "Dictionary Deck", "Deck created from CSV file");

        for entry in &self.entries {
            let note = Note::new(model.clone(), vec![&entry.question, &entry.answer])
                .map_err(|e| AnkiDictionaryError::NoteCreation(e.to_string()))?;
            deck.add_note(note);
        }

        let mut package = genanki_rs::Package::new(vec![deck], vec![])
            .map_err(|e| AnkiDictionaryError::Genanki(e.to_string()))?;
        package
            .write_to_file(
                path.as_ref()
                    .to_str()
                    .ok_or_else(|| AnkiDictionaryError::InvalidPath("Invalid path".to_string()))?,
            )
            .map_err(|e| AnkiDictionaryError::Genanki(e.to_string()))?;

        Ok(())
    }
}
