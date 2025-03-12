use chacha20_poly1305_aead::chacha20_encrypt;
use rand::Rng;
use x25519::{generate_public_key, generate_shared_secret};

use crate::hash::hash;

type PublicKey = [u8; 32];
type SecretKey = [u8; 32];

pub fn keygen() -> (PublicKey, SecretKey) {
    let sk = rand::rng().random::<[u8; 32]>();
    let pk = generate_public_key(&sk);
    (pk, sk)
}

pub const CIPHERTEXT_SIZE: usize = 64;

pub fn encrypt(
    plaintext: &[u8; 32],
    public_key: &PublicKey,
    randomness: &[u8; 32],
) -> [u8; CIPHERTEXT_SIZE] {
    let shared_secret = generate_shared_secret(randomness, public_key);
    let symmetric_key = hash::<32>(&shared_secret);

    let c1 = generate_public_key(randomness);

    let nonce = c1[..12].try_into().unwrap();
    let c2 = chacha20_encrypt(symmetric_key, nonce, 0, plaintext);

    let mut ciphertext = [0u8; CIPHERTEXT_SIZE];
    ciphertext[..32].copy_from_slice(&c1);
    ciphertext[32..].copy_from_slice(&c2);
    ciphertext
}

pub fn decrypt(ciphertext: &[u8; 64], secret_key: SecretKey) -> [u8; 32] {
    let (c1, c2) = ciphertext.split_at(32);

    let shared_secret = generate_shared_secret(&secret_key, c1.try_into().unwrap());
    let symmetric_key = hash::<32>(&shared_secret);

    let nonce = c1[..12].try_into().unwrap();
    chacha20_encrypt(symmetric_key, nonce, 0, c2)
        .try_into()
        .unwrap()
}
