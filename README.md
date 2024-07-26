# Anki Dictionary Creator

This Rust application creates an Anki deck from a CSV file containing question-answer pairs. It's designed to help users quickly generate Anki flashcards from their own dictionary or question-answer datasets.

## Features

- Reads question-answer pairs from a CSV file
- Generates an Anki deck compatible with Anki software
- Simple command-line interface

## Installation

- Clone the repository

- Build the project:
`cargo build --release`

The compiled binary will be available in the target/release directory.

## Usage

1. Prepare your CSV file with question-answer pairs. The CSV should have two columns: "Question" and "Answer".

1. Run the application:
`./target/release/anki_dictionary_creator <input_csv> <output_file>`

    Replace `<input_csv>` with the path to your input CSV file, and `<output_file>` with the desired path for the output Anki deck file (with a `.apkg` extension).

    Example:
    ```shell
    ./target/release/anki_dictionary_creator input.csv output.apkg
    ```

1. Import the generated `.apkg` file into Anki.
