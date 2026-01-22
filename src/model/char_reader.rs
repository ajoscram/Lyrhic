use std::{fs::File, path::PathBuf};
use utf8_read::{Char, Reader};

const COMMENT_START: char = '#';
const LINE_BREAK: char = '\n';

pub struct CharReader {
    path: PathBuf,
    reader: Reader<File>,
}

impl CharReader {
    pub fn new(path: &PathBuf) -> Self {
        let file = File::open(path).unwrap();
        let reader = Reader::new(file);
        Self { path: path.clone(), reader }
    }

    fn next_filtered_char(&mut self, current_char: Option<char>) -> Option<char> {
        match current_char {
            None => None,
            Some(char) =>  match char {
                COMMENT_START => { self.skip_line(); self.next() },
                char if is_skippable(char) => self.next(),
                char => Some(char),
            }
        }
    }

    fn skip_line(&mut self) {
        while let Some(char) = self.next_char() {
            if char == LINE_BREAK { break; }
        }
    }

    fn next_char(&mut self) -> Option<char> {
        match self.reader.next_char() {
            Err(error) => panic!("An error happened reading from {:?}: {}", &self.path, error),
            Ok(char_container) => match char_container {
                Char::Eof | Char::NoData => None,
                Char::Char(char) => Some(char),
            },
        }
    }
}

impl Clone for CharReader {
    fn clone(&self) -> Self { Self::new(&self.path) }
}

impl Iterator for CharReader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.next_char();
        self.next_filtered_char(first)
    }
}

fn is_skippable(char: char) -> bool { !char.is_ascii() || char.is_whitespace() || char.is_ascii_punctuation() }