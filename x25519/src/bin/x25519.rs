use std::{io, str::FromStr};

use clap::{arg, command, value_parser};
use x25519::{generate_public_key, generate_shared_secret};

#[derive(Clone)]
struct Bytes([u8; 32]);

// inspired from Thomas Sauvage's work
impl FromStr for Bytes {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s)
            .map_err(|_| {
                std::io::Error::new(io::ErrorKind::InvalidInput, "Hexadecimal string expected")
            })?
            .try_into()
            .and_then(|v| Ok(Self(v)))
            .map_err(|_| {
                std::io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid length. Expected {} bytes, got {}", 32, s.len() / 2),
                )
            })
    }
}

fn main() {
    let matches = command!()
        .arg(
            arg!(<m> "Scalar")
                .required(true)
                .value_parser(value_parser!(Bytes)),
        )
        .arg(
            arg!([u] "Vector")
                .required(false)
                .value_parser(value_parser!(Bytes)),
        )
        .get_matches();

    let Bytes(m) = matches.get_one("m").unwrap();
    let result = match matches.get_one("u") {
        Some(Bytes(u)) => generate_shared_secret(m, u),
        None => generate_public_key(m),
    };

    println!("{}", hex::encode(result));
}
