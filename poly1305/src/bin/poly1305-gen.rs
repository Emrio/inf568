use poly1305::poly1305_mac;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 2 {
        eprintln!("Usage: poly1305-gen <key> <filename>");
        std::process::exit(1);
    }

    let key = args[1].as_bytes();
    let key = hex::decode(key).unwrap();
    if key.len() != 32 {
        eprintln!("Key must be 32 bytes long");
        std::process::exit(1);
    }

    let filename = &args[2];
    let file_content = std::fs::read(filename).expect("Could not read file");

    let tag = poly1305_mac(key.try_into().unwrap(), &file_content);

    for byte in tag {
        print!("{:02x}", byte);
    }
    println!();
}
