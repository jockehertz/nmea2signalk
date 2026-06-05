use crate::signalk::{Delta, Update};
use crate::nmea0183::sentences::*;
use std::collections::HashMap;
use std::str::FromStr;

pub enum DataError {
    InvalidCharacter,
    InvalidFloat,
}

impl From<ParseFloatError> for DataError {
    fn from() -> DataError {
        DataError::InvalidFloat
    }
}

struct PendingMessage {
    data: String,
    parts_received: usize,
}

pub struct Nmea0183Parser {
    pending_messages: HashMap<u8, PendingMessage>,
}

/// An enum of ids for standard sentences
#[derive(Debug)]
pub enum StdSentenceId {
    Aam,
    Alm,
    Apa,
    Apb,
    Bod,
    Bwc,
    Bwr,
    Bww,
    Dbk,
    Dbs,
    Dbt,
    Dcn,
    Dpt,
    Dtm,
    Fsi,
    Gbs,
    Gga,
    Glc,
    Gll,
    Gns,
    Grs,
    Gst,
    Gsa,
    Gsv,
    Gtd,
    Gxa,
    Hdg,
    Hdm,
    Hdt,
    Hfb,
    Hsc,
    Its,
    Lcd,
    Mda,
    Msk,
    Mss,
    Mtw,
    Mwd,
    Mwv,
    Oln,
    Osd,
    R00,
    Rlm,
    Rma,
    Rmb,
    Rmc,
    Rot,
    Rpm,
    Rsa,
    Rsd,
    Rte,
    Sfi,
    Stn,
    Tds,
    Tfi,
    Tlb,
    Tll,
    Tpc,
    Tpr,
    Tpt,
    Trf,
    Ttm,
    Vbw,
    Vdr,
    Vhw,
    Vlw,
    Vpw,
    Vtg,
    Vwr,
    Wcv,
    Wnc,
    Wpl,
    Xdr,
    Xte,
    Xtr,
    Zda,
    Zfo,
    Ztg,
}

#[derive(Debug)]
pub enum AisId {
    Abm,
    Aca,
    Acs,
    Afi,
    Air,
    Vdm,
    Vdo,
    Vsd,
}

/// Errors in the sentences
#[derive(Debug)]
pub enum SentenceError {
    InvalidChecksum,
    InvalidStartChar,
    NonAsciiChar,
    EmptySentence,
    UnknownSentenceId,
}

/// Types of sentences
#[derive(Debug)]
pub enum SentenceType {
    Std(StdSentenceId),
    Ais(AisId),
}

/// The struct for a sentence
#[derive(Debug)]
pub struct Nmea0183Sentence {
    pub talker_id: String,
    pub kind: SentenceType,
    pub data: Vec<String>,
    pub checksum: u8,
}

/// Checks if the checksum is valid
fn validate_checksum(payload: &str, checksum: u8) -> bool {
    let mut calc: u8 = 0;
    for b in payload.bytes() {
        calc ^= b;
    }
    calc == checksum
}

/// Matches a sentence id to return a variant of SentenceId
fn match_std_sentence(id: &str) -> Result<StdSentenceId, SentenceError> {
    match id {
        "AAM" => Ok(StdSentenceId::Aam),
        "ALM" => Ok(StdSentenceId::Alm),
        "APA" => Ok(StdSentenceId::Apa),
        "APB" => Ok(StdSentenceId::Apb),
        "BOD" => Ok(StdSentenceId::Bod),
        "BWC" => Ok(StdSentenceId::Bwc),
        "BWR" => Ok(StdSentenceId::Bwr),
        "BWW" => Ok(StdSentenceId::Bww),
        "DBK" => Ok(StdSentenceId::Dbk),
        "DBS" => Ok(StdSentenceId::Dbs),
        "DBT" => Ok(StdSentenceId::Dbt),
        "DCN" => Ok(StdSentenceId::Dcn),
        "DPT" => Ok(StdSentenceId::Dpt),
        "DTM" => Ok(StdSentenceId::Dtm),
        "FSI" => Ok(StdSentenceId::Fsi),
        "GBS" => Ok(StdSentenceId::Gbs),
        "GGA" => Ok(StdSentenceId::Gga),
        "GLC" => Ok(StdSentenceId::Glc),
        "GLL" => Ok(StdSentenceId::Gll),
        "GNS" => Ok(StdSentenceId::Gns),
        "GRS" => Ok(StdSentenceId::Grs),
        "GST" => Ok(StdSentenceId::Gst),
        "GSA" => Ok(StdSentenceId::Gsa),
        "GSV" => Ok(StdSentenceId::Gsv),
        "GTD" => Ok(StdSentenceId::Gtd),
        "GXA" => Ok(StdSentenceId::Gxa),
        "HDG" => Ok(StdSentenceId::Hdg),
        "HDM" => Ok(StdSentenceId::Hdm),
        "HDT" => Ok(StdSentenceId::Hdt),
        "HFB" => Ok(StdSentenceId::Hfb),
        "HSC" => Ok(StdSentenceId::Hsc),
        "ITS" => Ok(StdSentenceId::Its),
        "LCD" => Ok(StdSentenceId::Lcd),
        "MDA" => Ok(StdSentenceId::Mda),
        "MSK" => Ok(StdSentenceId::Msk),
        "MSS" => Ok(StdSentenceId::Mss),
        "MTW" => Ok(StdSentenceId::Mtw),
        "MWD" => Ok(StdSentenceId::Mwd),
        "MWV" => Ok(StdSentenceId::Mwv),
        "OLN" => Ok(StdSentenceId::Oln),
        "OSD" => Ok(StdSentenceId::Osd),
        "R00" => Ok(StdSentenceId::R00),
        "RLM" => Ok(StdSentenceId::Rlm),
        "RMA" => Ok(StdSentenceId::Rma),
        "RMB" => Ok(StdSentenceId::Rmb),
        "RMC" => Ok(StdSentenceId::Rmc),
        "ROT" => Ok(StdSentenceId::Rot),
        "RPM" => Ok(StdSentenceId::Rpm),
        "RSA" => Ok(StdSentenceId::Rsa),
        "RSD" => Ok(StdSentenceId::Rsd),
        "RTE" => Ok(StdSentenceId::Rte),
        "SFI" => Ok(StdSentenceId::Sfi),
        "STN" => Ok(StdSentenceId::Stn),
        "TDS" => Ok(StdSentenceId::Tds),
        "TFI" => Ok(StdSentenceId::Tfi),
        "TLB" => Ok(StdSentenceId::Tlb),
        "TLL" => Ok(StdSentenceId::Tll),
        "TPC" => Ok(StdSentenceId::Tpc),
        "TPR" => Ok(StdSentenceId::Tpr),
        "TPT" => Ok(StdSentenceId::Tpt),
        "TRF" => Ok(StdSentenceId::Trf),
        "TTM" => Ok(StdSentenceId::Ttm),
        "VBW" => Ok(StdSentenceId::Vbw),
        "VDR" => Ok(StdSentenceId::Vdr),
        "VHW" => Ok(StdSentenceId::Vhw),
        "VLW" => Ok(StdSentenceId::Vlw),
        "VPW" => Ok(StdSentenceId::Vpw),
        "VTG" => Ok(StdSentenceId::Vtg),
        "VWR" => Ok(StdSentenceId::Vwr),
        "WCV" => Ok(StdSentenceId::Wcv),
        "WNC" => Ok(StdSentenceId::Wnc),
        "WPL" => Ok(StdSentenceId::Wpl),
        "XDR" => Ok(StdSentenceId::Xdr),
        "XTE" => Ok(StdSentenceId::Xte),
        "XTR" => Ok(StdSentenceId::Xtr),
        "ZDA" => Ok(StdSentenceId::Zda),
        "ZFO" => Ok(StdSentenceId::Zfo),
        "ZTG" => Ok(StdSentenceId::Ztg),
        _ => Err(SentenceError::UnknownSentenceId),
    }
}

fn match_ais_sentence(id: &str) -> Result<AisId, SentenceError> {
    match id {
        "ABM" => Ok(AisId::Abm),
        "ACA" => Ok(AisId::Aca),
        "ACS" => Ok(AisId::Acs),
        "AFI" => Ok(AisId::Afi),
        "AIR" => Ok(AisId::Air),
        "VDM" => Ok(AisId::Vdm),
        "VDO" => Ok(AisId::Vdo),
        "VSD" => Ok(AisId::Vsd),
        _ => Err(SentenceError::UnknownSentenceId),
    }
}
/// Parses the nmea0183 sentence
impl FromStr for Nmea0183Sentence {
    type Err = SentenceError;
    fn from_str(input: &str) -> Result<Nmea0183Sentence, SentenceError> {
        if !input.is_ascii() {
            return Err(SentenceError::NonAsciiChar);
        }

        if !input.starts_with("!") && !input.starts_with("$") {
            return Err(SentenceError::InvalidStartChar);
        }

        let checksum_split = input
            .split_once("*")
            .ok_or(SentenceError::InvalidChecksum)?;
        let text = checksum_split.0;

        let payload = &text[1..];
        // NMEA0183 sentences end with <CR><LF> (\r\n), this must be removed before parsing
        let checksum_str = checksum_split.1.trim_end();
        let checksum =
            u8::from_str_radix(checksum_str, 16).map_err(|_| SentenceError::InvalidChecksum)?;

        if !validate_checksum(payload, checksum) {
            return Err(SentenceError::InvalidChecksum);
        }


        let split: Vec<String> = payload.split(",").map(|s| s.to_string()).collect();

        let ids = &split[0];
        let talker = &ids[0..2];
        let sentence_type = if text.starts_with("$") {
            SentenceType::Std(match_std_sentence(&ids[2..])?)
        } else if text.starts_with("!") {
            SentenceType::Ais(match_ais_sentence(&ids[2..])?)
        } else {
            return Err(SentenceError::InvalidStartChar)
        };

        let data = split[1..].to_vec();

        Ok(Nmea0183Sentence {
            talker_id: talker.to_string(),
            kind: sentence_type,
            data,
            checksum,
        })
    }
}

/// Parses NMEA0183 raw string data to SignalK data
impl Nmea0183Parser {
    pub fn nmea0183_to_signalk(sentence: String) -> Delta {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_checksum() {
        // GPRMC with known good checksum
        assert!(validate_checksum("GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W", 0x6A));
    }

    #[test]
    fn test_invalid_checksum() {
        assert!(!validate_checksum("GPRMC,123519,A", 0x00));
    }
    #[test]
    fn test_parse_rmc() {
        let sentence = "$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A"
            .parse::<Nmea0183Sentence>();
        assert!(sentence.is_ok());
        let s = sentence.unwrap();
        assert_eq!(s.talker_id, "GP");
        assert!(matches!(s.kind, SentenceType::Std(StdSentenceId::Rmc)));
    }

    #[test]
    fn test_parse_ais() {
        let sentence = "!AIVDM,1,1,,A,15Muq@001oJr>tpE>f@EwvN20<0u,0*5B"
            .parse::<Nmea0183Sentence>();
        assert!(sentence.is_ok());
        let s = sentence.unwrap();
        assert_eq!(s.talker_id, "AI");
        assert!(matches!(s.kind, SentenceType::Ais(AisId::Vdm)));
    }

    #[test]
    fn test_invalid_checksum_error() {
        let result = "$GPRMC,123519*00".parse::<Nmea0183Sentence>();
        assert!(matches!(result, Err(SentenceError::InvalidChecksum)));
    }

    #[test]
    fn test_invalid_start_char() {
        let result = "GPRMC,123519*6A".parse::<Nmea0183Sentence>();
        assert!(matches!(result, Err(SentenceError::InvalidStartChar)));
    }
}
