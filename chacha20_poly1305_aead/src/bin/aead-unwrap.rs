use std::io::Write;

use chacha20_poly1305_aead::abort;
use chacha20_poly1305_aead::aead_unwrap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 6 {
        abort("Usage: aead_unwrap <key file> <nonce> <ad file> <input file> <tag>");
    }

    let key = match std::fs::read(&args[1]) {
        Ok(content) => content,
        Err(_) => abort("Could not read key file"),
    };
    if key.len() != 32 {
        abort("Key should have size 32 bytes");
    }
    let key = key.try_into().unwrap();

    let nonce = match hex::decode(args[2].as_bytes()) {
        Ok(nonce) => nonce,
        Err(_) => abort("Nonce should be a hex string"),
    };
    if nonce.len() != 12 {
        abort("Nonce should have have 12 bytes");
    }
    let nonce = nonce.try_into().unwrap();

    let aad = match std::fs::read(&args[3]) {
        Ok(content) => content,
        Err(_) => abort("Could not read input file"),
    };

    let ciphertext = match std::fs::read(&args[4]) {
        Ok(content) => content,
        Err(_) => abort("Could not read input file"),
    };

    let tag = match hex::decode(args[5].as_bytes()) {
        Ok(tag) => tag,
        Err(_) => abort("Tag should be a hex string"),
    };
    if tag.len() != 16 {
        abort("Tag should have have 16 bytes");
    }
    let tag = tag.try_into().unwrap();

    match aead_unwrap(key, nonce, &ciphertext, &aad, tag) {
        Some(plaintext) => std::io::stdout().write(&plaintext).unwrap(),
        None => std::process::exit(-1),
    };
}
