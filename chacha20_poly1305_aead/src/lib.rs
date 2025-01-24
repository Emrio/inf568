mod poly1305;

pub use poly1305::poly1305_mac;

pub fn abort(error: &str) -> ! {
    eprintln!("{error}");
    std::process::exit(1);
}
