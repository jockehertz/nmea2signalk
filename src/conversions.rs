use crate::nmea0183::parser::DataError;

pub fn ddmmmm_to_decimal_degrees(str: String) -> Result<f64, DataError> {
    let degrees = &str[0..2].parse::<f64>()?;
    let minutes = &str[2..].parse::<f64>()?;
    Ok(degrees + (minutes/60.0))
}

pub fn km_to_meters(dist: f64) -> f64 {
    dist*1000.0
}

pub fn nm_to_meters(dist: f64) -> f64 {
    dist*1852.0
}
