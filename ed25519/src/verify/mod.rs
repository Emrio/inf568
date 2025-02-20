use std::cmp::Ordering;

use num_bigint::BigUint;
use sha2::{Digest, Sha512};

use crate::arithmetic::constants;
use crate::arithmetic::Point;
use crate::keygen::Key;
use crate::sign::Signature;

pub fn verify(public_key: Key, message: &[u8], signature: Signature) -> bool {
    let Ok(a) = Point::from_bytes(public_key) else {
        return false;
    };

    let rs = signature[..32].try_into().unwrap();
    let Ok(r) = Point::from_bytes(rs) else {
        return false;
    };

    let s = BigUint::from_bytes_le(&signature[32..]);

    let l = &constants::l();

    if s.cmp(l) != Ordering::Less {
        // return Err("verify failed : s >= l");
        return false;
    }

    let h = Sha512::digest({
        let mut input = rs.to_vec();
        input.extend_from_slice(&public_key);
        input.extend_from_slice(&message);
        input
    });
    let h = BigUint::from_bytes_le(&h) % l;

    let sb = constants::b() * &s;
    let ha = a * &h;

    let rha = &r + &ha;

    sb == rha
}

#[cfg(test)]
mod tests;
