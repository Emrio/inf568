use std::process::exit;

use clap::{arg, command};
use ed25519::sign;
use ed25519::utils::read_private_key;

fn main() {
    let matches = command!()
        .arg(arg!(<prefix> "Key file prefix").required(true))
        .arg(arg!(<datafile> "Data file path").required(true))
        .arg(arg!(<sigfile> "Result signature file path").required(true))
        .get_matches();

    let prefix = matches.get_one::<String>("prefix").unwrap().to_owned();
    let datafile = matches.get_one::<String>("datafile").unwrap();
    let sigfile = matches.get_one::<String>("sigfile").unwrap();

    let private_key = read_private_key(prefix.to_owned() + ".sk");

    let Ok(message) = std::fs::read(datafile) else {
        eprintln!("Could not read data file");
        exit(1);
    };

    let signature = sign(private_key, &message);

    if let Err(_) = std::fs::write(sigfile, signature) {
        eprintln!("Could not write to signature file");
        exit(1);
    }
}
