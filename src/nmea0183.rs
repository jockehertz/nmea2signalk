use crate::signalk::{Update, Delta};


/// Errors in the sentences
pub enum SentenceError {
    InvalidChecksum,
    InvalidStartChar,
    NonAsciiChar,
    EmptySentence,
}

/// Types of sentences
pub enum SentenceType {
    Std,
    Ais
}

/// The struct for a sentence
pub struct Sentence {
    kind: SentenceType,
    data: Vec<String>,
    checksum: u8,
}

/// Checks if the checksum is valid
fn validate_checksum(payload: &str, checksum: u8) -> bool { 
    let mut calc: u8 = 0;
    for b in payload.bytes() {
        calc ^= b;
    }
    calc == checksum
}

/// Parses the nmea0183 sentence
fn parse_nmea0183(input: String) -> Result<Delta, SentenceError> {

    if ! input.is_ascii() {
        return Err(SentenceError::NonAsciiChar)
    }

    let checksum_split = input.split_once("*").ok_or(SentenceError::InvalidChecksum)?;
    let text = checksum_split.0;
    let payload = &text[1..];
    let checksum_str = checksum_split.1;
    let checksum = u8::from_str_radix(checksum_str, 16).map_err(|_| SentenceError::InvalidChecksum)?;

    if !validate_checksum(payload, checksum) {
        return Err(SentenceError::InvalidChecksum)
    }

    let kind = if text.starts_with("$") {
        SentenceType::Std
    } else if text.starts_with("!") {
        SentenceType::Ais
    } else {
        return Err(SentenceError::InvalidStartChar)
    };

    let split = payload.split(",").map(|s| s.to_string()).collect();

    Ok(/* Delta */)
}

/// Parses NMEA0183 raw string data to SignalK data
pub fn nmea0183_to_signalk(sentence: String) -> Delta {
    todo!()
}
