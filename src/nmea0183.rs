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

    let checksum_split = input.split_once("*").ok_or(SentenceError::InvalidChecksum)?;
    let text = checksum_split.0;
    let checksum_str = checksum_split.1;
    let checksum = u8::from_str_radix(checksum_str, 16).map_err(|_| SentenceError::InvalidChecksum)?;

    let split = text.split(",").map(|s| s.to_string()).collect();

    if ! input.is_ascii() {
        return Err(SentenceError::NonAsciiChar)
    }
    let kind = if input.starts_with("$") {
        SentenceType::Std
    } else if input.starts_with("!") {
        SentenceType::Ais
    } else {
        return Err(SentenceError::InvalidStartChar)
    };

    Ok(/* Delta */)
}
