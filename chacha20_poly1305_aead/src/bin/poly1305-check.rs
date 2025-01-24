use chacha20_poly1305_aead::abort;
use chacha20_poly1305_aead::poly1305_mac;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 3 {
        abort("Usage: poly1305-check <key> <filename> <tag>");
    }

    let key = match hex::decode(args[1].as_bytes()) {
        Ok(result) => result,
        Err(_) => abort("Key should be a hex string"),
    };
    if key.len() != 32 {
        abort("Key must be 32 bytes long");
    }
    let key = key.try_into().unwrap();

    let filename = &args[2];
    let file_content = match std::fs::read(filename) {
        Ok(content) => content,
        Err(_) => abort("Could not read file"),
    };

    let tag = hex::decode(args[3].as_bytes()).unwrap();
    if tag.len() != 16 {
        abort("Tag must be 16 bytes long");
    }
    let tag: [u8; 16] = tag.try_into().unwrap();

    let computed_tag = poly1305_mac(key, &file_content);
    if computed_tag != tag {
        println!("REJECT");
        std::process::exit(-1);
    }

    println!("ACCEPT");
}
