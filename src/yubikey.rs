use pcsc::*;
use std::ffi::CStr;

const YUBIKEY_NAME_FILTER: &str = "yubico yubikey";

const APDU_SELECT: &[u8] = b"\x00\xa4\x04\x00\x07\xa0\x00\x00\x05\x27\x21\x01";

#[derive(Debug)]
pub enum Error {
    NoYubikey,
    MoreThanOneYubikey,
    Connection,
    Transmission,
}

pub trait SmartCard {
    fn send_and_receive(&self, apdu: &[u8]) -> Result<Vec<u8>, Error>;
}

pub struct Yubikey {
    card: Card,
}

impl SmartCard for Yubikey {
    fn send_and_receive(&self, apdu: &[u8]) -> Result<Vec<u8>, Error> {
        send_and_receive(&self.card, apdu)
    }
}

impl Yubikey {
    pub fn initialize() -> Result<Self, Error> {
        let card = connect()?;
        send_and_receive(&card, APDU_SELECT)?;
        Ok(Self { card })
    }
}

fn connect() -> Result<Card, Error> {
    let ctx = Context::establish(Scope::User).map_err(|_| Error::Connection)?;

    let readers_buf_len = ctx.list_readers_len().map_err(|_| Error::Connection)?;
    if readers_buf_len > 4096 {
        return Err(Error::Connection);
    }
    let mut readers_buf = vec![0; readers_buf_len];

    let readers = ctx
        .list_readers(&mut readers_buf)
        .map_err(|_| Error::Connection)?;
    let filtered_readers = readers
        .filter(|r| match r.to_str() {
            Ok(name) => name
                .to_lowercase()
                .contains(&YUBIKEY_NAME_FILTER.to_lowercase()),
            Err(_) => false,
        })
        .collect::<Vec<&CStr>>();

    let reader = match filtered_readers.len() {
        0 => Err(Error::NoYubikey),
        1 => Ok(filtered_readers[0]),
        _ => Err(Error::MoreThanOneYubikey),
    }?;

    let card = ctx
        .connect(reader, ShareMode::Shared, Protocols::ANY)
        .map_err(|e| match e {
            pcsc::Error::NoSmartcard => Error::NoYubikey,
            _ => Error::Connection,
        })?;

    Ok(card)
}

fn send_and_receive(card: &Card, apdu: &[u8]) -> Result<std::vec::Vec<u8>, Error> {
    let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
    let rapdu = card
        .transmit(apdu, &mut rapdu_buf)
        .map_err(|_| Error::Transmission)?;
    Ok(rapdu.to_vec())
}
