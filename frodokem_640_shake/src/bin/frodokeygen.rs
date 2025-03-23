use std::process::exit;

use clap::{arg, command};
use frodokem_640_shake::frodokem::keygen;

fn main() {
    let matches = command!()
        .arg(arg!(<publickey> "Public key file path").required(true))
        .arg(arg!(<privatekey> "Private key file path").required(true))
        .get_matches();

    let public_key_file = matches.get_one::<String>("publickey").unwrap();
    let private_key_file = matches.get_one::<String>("privatekey").unwrap();

    let (public_key, private_key) = keygen();

    if let Err(_) = std::fs::write(public_key_file, public_key.to_bytes()) {
        eprintln!("Could not save public key file");
        exit(1);
    }

    if let Err(_) = std::fs::write(private_key_file, private_key.to_bytes()) {
        eprintln!("Could not save private key file");
        exit(1);
    }
}
