use clap::{arg, command, value_parser};
use curve25519_kem::{encaps, utils::Bytes};

fn main() {
    let matches = command!()
        .arg(
            arg!(<publickey> "Public key as hex string")
                .required(true)
                .value_parser(value_parser!(Bytes::<32>)),
        )
        .get_matches();

    let Bytes(public_key) = matches.get_one("publickey").unwrap();

    let (ciphertext, encryption_key) = encaps(*public_key);

    println!("{}", hex::encode(ciphertext));
    println!("{}", hex::encode(encryption_key));
}
