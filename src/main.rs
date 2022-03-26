#![feature(assert_matches)]

mod api;
mod oath;
mod time;
mod yubikey;

fn main() -> Result<(), api::Error> {
    api::serve()
}
