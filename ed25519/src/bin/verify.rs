use std::process::exit;

use clap::{arg, command};
use ed25519::utils::{read_public_key, read_signature};
use ed25519::verify;

fn main() {
    let matches = command!()
        .arg(arg!(<pkfile> "Public key file").required(true))
        .arg(arg!(<datafile> "Data file path").required(true))
        .arg(arg!(<sigfile> "Signature file path").required(true))
        .get_matches();

    let pkfile = matches.get_one::<String>("pkfile").unwrap().to_owned();
    let datafile = matches.get_one::<String>("datafile").unwrap();
    let sigfile = matches.get_one::<String>("sigfile").unwrap();

    let public_key = read_public_key(pkfile);

    let Ok(message) = std::fs::read(datafile) else {
        eprintln!("Could not read data file");
        exit(1);
    };

    let signature = read_signature(sigfile);

    if verify(public_key, &message, signature) {
        println!("ACCEPT")
    } else {
        println!("REJECT");
        exit(1)
    }
}
