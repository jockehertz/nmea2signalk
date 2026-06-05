// AAM - WAYPOINT ARRIVAL ALARM

use crate::signalk::{Delta, Update};
use crate::nmea0183::Nmea0183Sentence;
use crate::nmea0183::parser::DataError;
use crate::conversions::ddmmmm_to_decimal_degrees;


pub fn parse(sentence: Nmea0183Sentence) -> Result<Delta, DataError> {
    let data = sentence.data;

    let arrival_circle_entered: bool = match data[0].trim() {
        "A" => true,
        "V" => false,
        _ => return Err(DataError::InvalidCharacter)
    };

    let perpendicular_passed: bool = match data[1].trim() {
        "A" => true,
        "V" => false,
        _ => return Err(DataError::InvalidCharacter),
    };

    let arrival_circle_radius = data[2].trim().parse::<f64>()?;

    let latitude = ddmmmm_to_decimal_degrees(data[3])?;

    match data[4].trim() {
        "N" => (),
        "S" => latitude = -latitude,
        _ => return Err(DataError::InvalidCharacter),
    };

    let longitude = ddmmmm_to_decimal_degrees(data[5])?;

    match data[6].trim() {
        "E" => (),
        "W" => longitude = -longitude,
    };

}
