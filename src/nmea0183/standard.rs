use crate::signalk::{Delta, Update};
use std::collections::HashMap;
use std::str::FromStr;

struct PendingMessage {
    data: String,
    counter: usize,
}
pub struct Nmea0183Parser {
    pending_messages: HashMap<u8, PendingMessage>,
}

/// An enum of ids for standard sentences
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

pub enum AisId {
    Own,
    Other,
}

/// Errors in the sentences
pub enum SentenceError {
    InvalidChecksum,
    InvalidStartChar,
    NonAsciiChar,
    EmptySentence,
    UnknownSentenceId,
}

/// Types of sentences
pub enum SentenceType {
    Std(StdSentenceId),
    Ais(AisId),
}

/// The struct for a sentence
pub struct Nmea0183Sentence {
    talker_id: String,
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

/// Matches a sentence id to return a variant of SentenceId
fn match_sentence(id: &str) -> Result<StdSentenceId, SentenceError> {
    match id {
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

        let kind = if text.starts_with("$") {
            SentenceType::Std
        } else if text.starts_with("!") {
            SentenceType::Ais
        } else {
            return Err(SentenceError::InvalidStartChar);
        };

        let split = payload.split(",").map(|s| s.to_string()).collect();

        let ids = payload[0];
        let talker = ids[0..2];
        let sentence_id = match_sentence(ids[2..])?;

        Ok(/* Nmea0183Sentence */)
    }
}

/// Parses NMEA0183 raw string data to SignalK data
impl Nmea0183Parser {
    pub fn nmea0183_to_signalk(sentence: String) -> Delta {
        todo!()
    }
}
