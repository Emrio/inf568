use std::process::exit;

use crate::{keygen::Key, sign::Signature};

pub fn read_private_key(path: String) -> Key {
    let Ok(private_key) = std::fs::read(path) else {
        eprintln!("Could not read private key file");
        exit(1);
    };

    let Ok(private_key) = private_key[..].try_into() else {
        eprintln!(
            "Invalid private key length: expected 32 bytes, got {}",
            private_key.len()
        );
        exit(1);
    };

    private_key
}

pub fn read_public_key(path: String) -> Key {
    let Ok(public_key) = std::fs::read(path) else {
        eprintln!("Could not read public key file");
        exit(1);
    };

    let Ok(public_key) = public_key[..].try_into() else {
        eprintln!(
            "Invalid public key length: expected 32 bytes, got {}",
            public_key.len()
        );
        exit(1);
    };

    public_key
}

pub fn read_signature(path: &str) -> Signature {
    let Ok(signature) = std::fs::read(path) else {
        eprintln!("Could not read signature file");
        exit(1);
    };

    let Ok(signature) = signature[..].try_into() else {
        eprintln!(
            "Invalid signature length: expected 64 bytes, got {}",
            signature.len()
        );
        exit(1);
    };

    signature
}
