use crate::chacha20;
use crate::poly1305;

fn poly1305_key_gen(key: chacha20::Key, nonce: chacha20::Nonce) -> poly1305::Key {
    chacha20::chacha20_block(key, nonce, 0)[..32]
        .try_into()
        .unwrap()
}

fn pad16(message: &mut Vec<u8>) {
    while message.len() % 16 != 0 {
        message.push(0);
    }
}

fn prepare_message(ciphertext: &[u8], aad: &[u8]) -> Vec<u8> {
    let mut message = Vec::with_capacity(aad.len() + 15 + ciphertext.len() + 15 + 8 + 8);

    message.extend_from_slice(aad);
    pad16(&mut message);

    message.extend_from_slice(&ciphertext);
    pad16(&mut message);

    message.extend_from_slice(&u64::to_le_bytes(aad.len() as u64));
    message.extend_from_slice(&u64::to_le_bytes(ciphertext.len() as u64));

    message
}

pub fn wrap(
    key: chacha20::Key,
    nonce: chacha20::Nonce,
    plaintext: &[u8],
    aad: &[u8],
) -> (poly1305::Tag, Vec<u8>) {
    let ciphertext = chacha20::encrypt(key, nonce, 1, plaintext);

    let message = prepare_message(&ciphertext, aad);
    let key = poly1305_key_gen(key, nonce);
    let tag = poly1305::mac(key, &message);

    (tag, ciphertext)
}

pub fn unwrap(
    key: chacha20::Key,
    nonce: chacha20::Nonce,
    ciphertext: &[u8],
    aad: &[u8],
    tag: poly1305::Tag,
) -> Option<Vec<u8>> {
    let decrypted = chacha20::encrypt(key, nonce, 1, ciphertext);

    let message = prepare_message(&ciphertext, aad);
    let key = poly1305_key_gen(key, nonce);
    let computed_tag = poly1305::mac(key, &message);

    if tag == computed_tag {
        Some(decrypted)
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
