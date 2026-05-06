use crate::signalk::{Update, Delta};


pub enum SentenceError {
    InvalidChecksum,
    InvalidStartChar,
    NonAsciiChar,
    EmptySentence,
}

pub enum SentenceType {
    Std,
    Ais
}
pub struct Sentence {
    kind: SentenceType,
    data: Vec<String>,
    checksum: u8,
}

fn checksum(sentence: Sentence) -> bool { 
    todo!()
}

pub fn parse_nmea0183(input: String) -> Result<Delta, SentenceError> {

    let checksum_split = input.split("*").map(|s| s.to_string()).collect();
    let text = checksum_split[0];
    let checksum = checksum_split[1];

    let split = text.split(",").map(|s| s.to_string()).collect();

    if ! input.is_ascii() {
        return Err(SentenceError::NonAsciiChar)
    }
    let sentence = if input.starts_with("$") {
        Sentence::Std(split)
    } else if input.starts_with("!") {
        Sentence::Ais(split)
    } else {
        return Err(SentenceError::InvalidStartChar)
    }

    Ok(/* Delta */)
}
