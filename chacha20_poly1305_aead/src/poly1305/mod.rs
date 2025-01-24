use num_bigint::{BigUint, ToBigUint};

fn clamp(r: BigUint) -> BigUint {
    let mask = 0x0ff_ffffc_0fff_fffc_0fff_fffc_0fff_ffffu128
        .to_biguint()
        .unwrap();
    r & mask
}

fn parse_key(key: [u8; 32]) -> (BigUint, BigUint) {
    let (r, s) = key.split_at(16);
    let r = BigUint::from_bytes_le(r);
    let s = BigUint::from_bytes_le(s);
    (clamp(r), s)
}

pub fn poly1305_mac(key: [u8; 32], message: &[u8]) -> [u8; 16] {
    let (r, s) = &parse_key(key);

    let mut a = BigUint::new(vec![0]);
    let p = &(2u128.to_biguint().unwrap().pow(130) - 5.to_biguint().unwrap());

    for block in message.chunks(16) {
        let mut n = BigUint::from_bytes_le(block);
        n += 2u128.to_biguint().unwrap().pow(block.len() as u32 * 8);
        a = ((a + n) * r) % p;
    }

    let result = (a + s).to_bytes_le();
    result[..16].try_into().unwrap()
}

#[cfg(test)]
mod tests;
