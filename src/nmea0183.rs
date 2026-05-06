use crate::signalk::{Update, Delta};

const VALID_STARTS: [&str; 2] = ["$", "!"];

pub enum SentenceError {
    InvalidChecksum,
    InvalidStartChar,
    NonAsciiChar,
    EmptySentence,
}

pub enum Sentence {
    Std(Vec<String>),
    Ais(Vec<String>)
}

fn checksum(sentence) -> bool { 
    todo!()
}

pub fn parse_nmea0183(input: String) -> Result<Delta, SentenceError> {

    let split = input.split(",");

    if ! sentence.is_ascii() {
        return Err(SentenceError::NonAsciiChar)
    }
    if input.starts_with("$") {
        let sentence = Sentence::Std(split);
    } else if input.starts_with("!") {
        let sentence = Sentence::Ais(split);
    } else {
        return Err(SentenceError::InvalidStartChar)
    }

    Ok(/* Delta */)
}
