use std::process::exit;

use clap::{arg, command};
use ed25519::keygen;

fn main() {
    let matches = command!()
        .arg(arg!(<prefix> "Key file prefix").required(true))
        .get_matches();

    let prefix = matches.get_one::<String>("prefix").unwrap().to_owned();

    let (private_key, public_key) = keygen();

    if let Err(_) = std::fs::write(prefix.clone() + ".sk", private_key) {
        eprintln!("Could not save private key file");
        exit(1);
    }

    if let Err(_) = std::fs::write(prefix + ".pk", public_key) {
        eprintln!("Could not save public key file");
        exit(1);
    }
}
