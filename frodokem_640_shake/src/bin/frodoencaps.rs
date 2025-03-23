use std::process::exit;

use clap::{arg, command};
use frodokem_640_shake::frodokem::{PublicKey, encaps};

fn main() {
    let matches = command!()
        .arg(arg!(<publickey> "Public key file path").required(true))
        .arg(arg!(<ciphertext> "Ciphertext file path").required(true))
        .arg(arg!(<sharedsecret> "Shared secret file path").required(true))
        .get_matches();

    let public_key_file = matches.get_one::<String>("publickey").unwrap();
    let ciphertext_file = matches.get_one::<String>("ciphertext").unwrap();
    let shared_secret_file = matches.get_one::<String>("sharedsecret").unwrap();

    let Ok(public_key) = std::fs::read(public_key_file) else {
        eprintln!("Could not read public key file");
        exit(1);
    };

    let (ciphertext, shared_secret) = encaps(PublicKey::from_bytes(&public_key));

    if let Err(_) = std::fs::write(ciphertext_file, ciphertext.to_bytes()) {
        eprintln!("Could not save public key file");
        exit(1);
    }

    if let Err(_) = std::fs::write(shared_secret_file, shared_secret) {
        eprintln!("Could not save private key file");
        exit(1);
    }
}
