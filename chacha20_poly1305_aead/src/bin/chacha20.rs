use chacha20_poly1305_aead::abort;
use chacha20_poly1305_aead::chacha20_encrypt;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        abort("Usage: chacha20 <key file> <nonce> <input file> <output file>");
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

    let plaintext = match std::fs::read(&args[3]) {
        Ok(content) => content,
        Err(_) => abort("Could not read input file"),
    };

    let output = chacha20_encrypt(key, nonce, 1, &plaintext[..]);
    match std::fs::write(&args[4], output) {
        Err(_) => abort("Could not write to file"),
        _ => (),
    }
}
