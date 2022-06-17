use crate::yubikey;

// see documentation at
//   - https://developers.yubico.com/OATH/YKOATH_Protocol.html
//   - https://docs.yubico.com/yesdk/users-manual/application-oath/oath-commands.html

const APDU_LIST: &[u8] = b"\x00\xa1\x00\x00";
const APDU_REMAINING: &[u8] = b"\x00\xa5\x00\x00";
const APDU_CALCULATE: &[u8] = b"\x00\xa2\x00\x01";

const TAG_NAME: u8 = b'\x71';
const TAG_NAME_LIST: u8 = b'\x72';
const TAG_CHALLENGE: u8 = b'\x74';
const TAG_RESPONSE_FULL: u8 = b'\x75';
const TAG_RESPONSE_TRUNCATED: u8 = b'\x76';

const STATUS_MORE_DATA: u8 = b'\x61';
const STATUS_AUTH_REQUIRED: &[u8] = b"\x6982";
const STATUS_NO_SUCH_OBJECT: &[u8] = b"\x6984";

const OATH_INTERVAL_SECONDS: u64 = 30;

#[derive(Debug)]
pub enum Error {
    Yubikey,
    NoMatchingCredential,
    TooManyMatchingCredentials,
    AuthRequired,
}

pub fn list_credentials(yubikey: &impl yubikey::SmartCard) -> Result<Vec<String>, Error> {
    let mut raw_creds: Vec<u8> = Vec::new();
    let mut response = yubikey
        .send_and_receive(APDU_LIST)
        .map_err(|_| Error::Yubikey)?;

    while response[response.len() - 2] == STATUS_MORE_DATA {
        raw_creds.extend(response.iter().take(response.len() - 2));
        response = yubikey
            .send_and_receive(APDU_REMAINING)
            .map_err(|_| Error::Yubikey)?;
    }

    raw_creds.append(&mut response);

    Ok(parse_credentials(&raw_creds))
}

pub fn calculate(yubikey: &impl yubikey::SmartCard, cred: &str, time: u64) -> Result<u32, Error> {
    let cred_bytes = cred.as_bytes();
    let challenge = time / OATH_INTERVAL_SECONDS;
    let challenge_bytes = challenge.to_be_bytes();
    let data_len = (cred_bytes.len() + challenge_bytes.len() + 4) as u8;

    let apdu = [
        APDU_CALCULATE,
        &[data_len],
        &[TAG_NAME],
        &[cred_bytes.len() as u8],
        cred_bytes,
        &[TAG_CHALLENGE],
        &[challenge_bytes.len() as u8],
        &challenge_bytes,
    ]
    .concat();

    let rapdu = yubikey
        .send_and_receive(&apdu)
        .map_err(|_| Error::Yubikey)?;
    let rapdu_len = rapdu.len();

    if rapdu_len == 0 {
        return Err(Error::Yubikey);
    }

    if rapdu.starts_with(STATUS_AUTH_REQUIRED) {
        return Err(Error::AuthRequired);
    }

    if rapdu.starts_with(STATUS_NO_SUCH_OBJECT) {
        return Err(Error::NoMatchingCredential);
    }

    if !(rapdu.starts_with(&[TAG_RESPONSE_FULL]) || rapdu.starts_with(&[TAG_RESPONSE_TRUNCATED])) {
        return Err(Error::Yubikey);
    }

    if rapdu_len < 7 {
        Err(Error::Yubikey)
    } else {
        Ok(u32::from_be_bytes([rapdu[3], rapdu[4], rapdu[5], rapdu[6]]))
    }
}

pub fn calculate_fuzzy(
    yubikey: &impl yubikey::SmartCard,
    search_term: &str,
    time: u64,
) -> Result<u32, Error> {
    let search_term_lower = search_term.to_lowercase();

    let creds = list_credentials(yubikey).map_err(|_| Error::Yubikey)?;

    let matching_creds = creds
        .iter()
        .filter(|cred| cred.to_lowercase().contains(&search_term_lower))
        .collect::<Vec<&String>>();

    match matching_creds.len() {
        0 => Err(Error::NoMatchingCredential),
        1 => calculate(yubikey, matching_creds[0], time),
        _ => Err(Error::TooManyMatchingCredentials),
    }
}

fn parse_credentials(rapdu: &[u8]) -> Vec<String> {
    let mut creds: Vec<String> = Vec::new();
    let mut buf_it = rapdu.iter();

    while buf_it.next() == Some(&TAG_NAME_LIST) {
        let len = match buf_it.next() {
            Some(len) => (*len - 1) as usize,
            _ => break,
        };
        buf_it.next(); // skip algorithm
        let cred_it = buf_it.by_ref().take(len);
        let cred_bytes = cred_it.cloned().collect::<Vec<u8>>();
        let cred = match std::str::from_utf8(&cred_bytes) {
            Ok(cred) => cred,
            Err(_) => break,
        };
        creds.push(cred.to_owned());
    }

    creds
}
