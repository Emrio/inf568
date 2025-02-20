use num_bigint::BigUint;
use sha2::{Digest, Sha512};

use crate::arithmetic::constants;
use crate::keygen::derivate_public_key_scalar_prefix;

pub type Signature = [u8; 64];

pub fn sign(private_key: [u8; 32], message: &[u8]) -> Signature {
    let (public_key, s, prefix) = derivate_public_key_scalar_prefix(private_key);
    let l = &constants::l();
    let p = &constants::p();

    let r = {
        let mut input = prefix.to_vec();
        input.extend_from_slice(message);
        Sha512::digest(input)
    };
    let r = BigUint::from_bytes_le(&r) % l;

    let b = constants::b();
    let r2 = (b * &r).to_bytes();

    let k = {
        let mut input = r2.to_vec();
        input.extend_from_slice(&public_key);
        input.extend_from_slice(&message);
        Sha512::digest(input)
    };
    let k = BigUint::from_bytes_le(&k) % l;

    let s2 = ((r + k * s) % l) % p;
    let s2 = s2.to_bytes_le();

    let mut input = r2.to_vec();
    input.extend_from_slice(&s2);
    input.resize(64, 0);
    input.try_into().unwrap()
}

#[cfg(test)]
mod tests;
