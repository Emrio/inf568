use poly1305::poly1305_mac;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 3 {
        eprintln!("Usage: poly1305-check <key> <filename> <tag>");
        std::process::exit(1);
    }

    let key = args[1].as_bytes();
    let key = hex::decode(key).unwrap();
    if key.len() != 32 {
        eprintln!("Key must be 32 bytes long");
        std::process::exit(1);
    }

    let filename = &args[2];
    let file_content = match std::fs::read(filename) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Could not read file");
            std::process::exit(1);
        }
    };

    let tag = args[3].as_bytes();
    let tag = hex::decode(tag).unwrap();
    if tag.len() != 16 {
        eprintln!("Tag must be 16 bytes long");
        std::process::exit(1);
    }

    let computed_tag = poly1305_mac(key.try_into().unwrap(), &file_content);
    if computed_tag != <[u8; 16]>::try_from(tag).unwrap() {
        println!("REJECT");
        std::process::exit(-1);
    }

    println!("ACCEPT");
}
