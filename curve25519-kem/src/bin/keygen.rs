use std::process::exit;

use clap::{arg, command};
use curve25519_kem::keygen;

fn main() {
    let matches = command!()
        .arg(arg!(<privatekey> "Private key file path").required(true))
        .get_matches();

    let private_key_file = matches.get_one::<String>("privatekey").unwrap();

    let (public_key, private_key) = keygen();

    if let Err(_) = std::fs::write(private_key_file, private_key.to_bytes()) {
        eprintln!("Could not save private key file");
        exit(1);
    }

    println!("{}", hex::encode(public_key));
}
