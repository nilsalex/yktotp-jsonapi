use crate::yubikey;

const APDU_LIST: &[u8] = b"\x00\xa1\x00\x00";
const APDU_REMAINING: &[u8] = b"\x00\xa5\x00\x00";
const APDU_CALCULATE: &[u8] = b"\x00\xa2\x00\x01";

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

    while response[response.len() - 2] == 0x61 {
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
    let challenge = time / 30;
    let challenge_bytes = challenge.to_be_bytes();
    let apdu = [
        APDU_CALCULATE,
        &[cred_bytes.len() as u8 + 12],
        &[0x71],
        &[cred_bytes.len() as u8],
        cred_bytes,
        &[0x74],
        &[8],
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

    if rapdu.starts_with(b"\x6982") {
        return Err(Error::AuthRequired);
    }

    if rapdu.starts_with(b"\x6984") {
        return Err(Error::NoMatchingCredential);
    }

    if !rapdu.starts_with(b"\x76") {
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

    while let Some(0x72) = buf_it.next() {
        let len = match buf_it.next() {
            Some(len) => (*len - 1) as usize,
            _ => break,
        };
        buf_it.next();
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
