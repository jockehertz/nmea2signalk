use crate::signalk::{Update, Delta};

const VALID_STARTS: [&str; 2] = ["$", "!"];

pub enum SentenceError {
    InvalidChecksum,
    InvalidStartChar,
    NonAsciiChar,
    EmptySentence,
}

fn checksum(sentence) -> bool { 
    todo!()
}

pub fn parse_nmea0183(sentence: String) -> Result<Delta, SentenceError> {
    let start = match sentence.chars().nth(0) {
        Some(c) => c,
        None => return Err(SentenceError::EmptySentence),
    };

    let split = sentence.split(",");

    if ! sentence.is_ascii() {
        return Err(SentenceError::NonAsciiChar)
    }
    if ! VALID_STARTS.contains(&start.to_string().as_str()) {
        return Err(SentenceError::InvalidStartChar)
    }

    Ok(/* Delta */)
}
