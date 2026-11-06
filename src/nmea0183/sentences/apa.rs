use crate::signalk::Delta;
use serde_json::Value;
use crate::nmea0183::parser::DataError;
use crate::nmea0183::Nmea0183Sentence;
use crate::conversions::{nm_to_meters, km_to_meters};

pub fn apa(sentence: Nmea0183Sentence) -> Result<Option<Delta>, DataError> {
    let data = sentence.data;
    match data[0].trim() {
        "A" => (),
        "V" => return Err(DataError::NavigationWarning),
    };

    match data[1].trim() {
        "A" => (),
        "V" => return Err(DataError::NavigationWarning),
    };

    let mut xte = data[2].trim().parse::<f64>()?;

    match data[3].trim() {
        "R" => (),
        "L" => xte = -xte,
    };

    match data[4].trim() {
        "N" => xte = nm_to_meters(xte),
        "K" => xte = km_to_meters(xte),
    };

    let arrival_circle_entered = match data[5].trim() {
        "A" => true,
        "V" => false,
        _ => return Err(DataError::InvalidCharacter),
    };

    let perpendicular_passed = match data[6].trim() {
        "A" => true,
        "V" => false,
        _ => return Err(DataError::InvalidCharacter),
    };

}
