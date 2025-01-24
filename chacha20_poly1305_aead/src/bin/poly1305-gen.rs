use chacha20_poly1305_aead::abort;
use chacha20_poly1305_aead::poly1305_mac;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 2 {
        abort("Usage: poly1305-gen <key> <filename>");
    }

    let key = args[1].as_bytes();
    let key = hex::decode(key).unwrap();
    if key.len() != 32 {
        abort("Key must be 32 bytes long");
    }
    let key = key.try_into().unwrap();

    let file_content = match std::fs::read(&args[2]) {
        Ok(content) => content,
        Err(_) => abort("Could not read file"),
    };

    let tag = poly1305_mac(key, &file_content);

    for byte in tag {
        print!("{:02x}", byte);
    }
    println!();
}
