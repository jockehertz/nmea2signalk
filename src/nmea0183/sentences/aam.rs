// AAM - WAYPOINT ARRIVAL ALARM

use crate::nmea0183::parser::DataError;
use crate::nmea0183::Nmea0183Sentence;
use crate::signalk::{Delta};

pub fn parse(sentence: Nmea0183Sentence) -> Result<Option<Delta>, DataError> {
    let data = sentence.data;

    let arrival_circle_entered: bool = match data[0].trim() {
        "A" => true,
        "V" => false,
        _ => return Err(DataError::InvalidCharacter),
    };

    let perpendicular_passed: bool = match data[1].trim() {
        "A" => true,
        "V" => false,
        _ => return Err(DataError::InvalidCharacter),
    };

    if !arrival_circle_entered && !perpendicular_passed {
        return Ok(None);
    }

    let mut delta = Delta::new("vessels.self");

    if arrival_circle_entered {
        delta = delta.add_update(
            "notifications.navigation.course.arrivalCircleEntered",
            serde_json::json!(arrival_circle_entered),
        );
    }

    if perpendicular_passed {
        delta = delta.add_update(
            "notifications.navigation.course.perpendicularPassed",
            serde_json::json!(perpendicular_passed),
        );
    }

    Ok(Some(delta))
}
