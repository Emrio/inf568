use num_bigint::BigUint;
use rand::fill;
use sha2::{Digest, Sha512};

use crate::arithmetic::constants;

pub type Key = [u8; 32];

fn generate_private_key() -> Key {
    let mut key = [0u8; 32];
    fill(&mut key);
    key
}

fn prune_buffer(buffer: &mut Key) {
    buffer[0] &= 0xf8;
    buffer[31] &= 0x7f;
    buffer[31] |= 0x40;
}

pub fn derivate_public_key_scalar_prefix(private_key: Key) -> (Key, BigUint, Key) {
    let result = Sha512::digest(private_key);
    let (s, prefix) = result.split_at(32);

    let mut s = s.try_into().unwrap();
    prune_buffer(&mut s);
    let s = BigUint::from_bytes_le(&s);

    let b = constants::b();
    let a = b * &s;

    (a.to_bytes(), s, prefix.try_into().unwrap())
}

pub fn keygen() -> (Key, Key) {
    let private_key = generate_private_key();
    let (public_key, _, _) = derivate_public_key_scalar_prefix(private_key);
    (private_key, public_key)
}

#[cfg(test)]
mod tests;
