use num_bigint::{BigUint, ToBigUint};

fn decode_u_coordinate(mut u: [u8; 32]) -> BigUint {
    u[31] &= (1 << 7) - 1;

    BigUint::from_bytes_le(&u)
}

fn encode_u_coordinate(u: BigUint, p: &BigUint) -> [u8; 32] {
    let u = u % p;
    let mut u = BigUint::to_bytes_le(&u);
    u.resize(32, 0);

    u.try_into().unwrap()
}

fn decode_scalar(mut k: [u8; 32]) -> BigUint {
    k[0] &= 248;
    k[31] &= 127;
    k[31] |= 64;

    BigUint::from_bytes_le(&k)
}

fn cswap(test: bool, a: &mut BigUint, b: &mut BigUint, c: &mut BigUint, d: &mut BigUint) {
    let mask = (test as u8).wrapping_neg();
    let mask = BigUint::from_bytes_le(&[mask; 32]);

    let dummy = &mask & (&*a ^ &*b);
    *a ^= &dummy;
    *b ^= &dummy;

    let dummy = mask & (&*c ^ &*d);
    *c ^= &dummy;
    *d ^= &dummy;
}

fn ladder(k: &BigUint, u: &BigUint, p: &BigUint) -> BigUint {
    let x1 = u;
    let mut x2 = BigUint::from(1u8);
    let mut z2 = BigUint::ZERO;
    let mut x3 = u.clone();
    let mut z3 = BigUint::from(1u8);
    let mut swap = false;
    let a24 = BigUint::from(121665u32);

    for t in (0..=254).rev() {
        let kt = k.bit(t);
        swap ^= kt;

        cswap(swap, &mut x2, &mut x3, &mut z2, &mut z3);

        swap = kt;

        let a = &x2 + &z2;
        let aa = &a.pow(2) % p;
        let b = (x2 + p) - z2;
        let bb = &b.pow(2) % p;
        let e = (p + &aa) - &bb;
        let c = &x3 + &z3;
        let d = (x3 + p) - &z3;
        let da = (d * a) % p;
        let cb = (c * b) % p;
        x3 = ((&da + &cb).pow(2)) % p;
        z3 = (x1 * ((&da + p) - &cb).pow(2)) % p;
        x2 = (&bb * &aa) % p;
        z2 = ((&aa + &a24 * &e) * &e) % p;
    }

    cswap(swap, &mut x2, &mut x3, &mut z2, &mut z3);

    (x2 * (z2.modpow(&(p - 2.to_biguint().unwrap()), p))) % p
}

fn x25519(k: &[u8; 32], u: &[u8; 32]) -> [u8; 32] {
    let k = decode_scalar(k.clone());
    let u = decode_u_coordinate(u.clone());

    let p = BigUint::from(2u32).pow(255) - BigUint::from(19u32);
    let u = ladder(&k, &u, &p);

    encode_u_coordinate(u, &p)
}

const BASE_POINT: [u8; 32] = {
    let mut base_point = [0; 32];
    base_point[0] = 9;
    base_point
};

pub fn generate_public_key(sk: &[u8; 32]) -> [u8; 32] {
    x25519(sk, &BASE_POINT)
}

pub fn generate_shared_secret(sk: &[u8; 32], peer_pk: &[u8; 32]) -> [u8; 32] {
    x25519(sk, peer_pk)
}

#[cfg(test)]
mod tests;
