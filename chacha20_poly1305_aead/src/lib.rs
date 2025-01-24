mod chacha20;
mod poly1305;

pub use chacha20::chacha20_encrypt;
pub use poly1305::poly1305_mac;

pub fn abort(error: &str) -> ! {
    eprintln!("{error}");
    std::process::exit(1);
}
