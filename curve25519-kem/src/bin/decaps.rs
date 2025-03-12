use std::process::exit;

use clap::{arg, command, value_parser};
use curve25519_kem::{decaps, utils::Bytes, SecretKey};

fn main() {
    let matches = command!()
        .arg(arg!(<privatekey> "Private key file path").required(true))
        .arg(
            arg!(<ciphertext> "Cipher text as hex string")
                .required(true)
                .value_parser(value_parser!(Bytes::<64>)),
        )
        .get_matches();

    let private_key_file = matches.get_one::<String>("privatekey").unwrap();
    let Bytes(ciphertext) = matches.get_one("ciphertext").unwrap();

    let Ok(private_key) = std::fs::read(private_key_file) else {
        eprintln!("Could not read private key file");
        exit(1);
    };

    let encryption_key = decaps(*ciphertext, SecretKey::from_bytes(&private_key[..]));

    println!("{}", hex::encode(encryption_key));
}
