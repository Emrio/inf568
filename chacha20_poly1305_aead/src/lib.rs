mod aead;
mod chacha20;
mod poly1305;

pub use aead::unwrap as aead_unwrap;
pub use aead::wrap as aead_wrap;
pub use chacha20::encrypt as chacha20_encrypt;
pub use poly1305::mac as poly1305_mac;

pub fn abort(error: &str) -> ! {
    eprintln!("{error}");
    std::process::exit(1);
}
