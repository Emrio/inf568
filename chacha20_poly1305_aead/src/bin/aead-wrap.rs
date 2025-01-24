use chacha20_poly1305_aead::abort;
use chacha20_poly1305_aead::aead_wrap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 6 {
        abort("Usage: aead_wrap <key file> <nonce> <ad file> <input file> <output file>");
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

    let plaintext = match std::fs::read(&args[4]) {
        Ok(content) => content,
        Err(_) => abort("Could not read input file"),
    };

    let (tag, ciphertext) = aead_wrap(key, nonce, &plaintext, &aad);
    match std::fs::write(&args[5], ciphertext) {
        Err(_) => abort("Could not write to file"),
        _ => (),
    }

    println!("{}", hex::encode(tag));
}
