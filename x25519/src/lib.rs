use num_bigint::{BigUint, ToBigUint};

fn decode_u_coordinate(mut u: Vec<u8>) -> BigUint {
    assert!(u.len() > 0);

    let last = u.last_mut().unwrap();
    *last &= (1 << 7) - 1;

    BigUint::from_bytes_le(&u)
}

fn encode_u_coordinate(u: BigUint) -> Vec<u8> {
    let p = BigUint::from(2u32).pow(255) - BigUint::from(19u32);
    let u = u % p;
    BigUint::to_bytes_le(&u)
}

fn decode_scalar(mut k: Vec<u8>) -> BigUint {
    assert_eq!(k.len(), 32);

    k[0] &= 248;
    k[31] &= 127;
    k[31] |= 64;

    BigUint::from_bytes_le(&k)
}

fn cswap(test: bool, a: BigUint, b: BigUint) -> (BigUint, BigUint) {
    let mask: u8 = if test { 0xFF } else { 0 };
    let mask = BigUint::from_bytes_le(&[mask; 32]);
    let dummy = mask & (&a ^ &b);
    (a ^ &dummy, b ^ &dummy)
}

fn x25519(k: &BigUint, u: &BigUint) -> BigUint {
    let x1 = u;
    let mut x2 = BigUint::from(1u8);
    let mut z2 = BigUint::ZERO;
    let mut x3 = u.clone();
    let mut z3 = BigUint::from(1u8);
    let mut swap = false;
    let a24 = BigUint::from(121665u32);
    let p = BigUint::from(2u32).pow(255) - BigUint::from(19u32);

    for t in (0..=254).rev() {
        let kt = k.bit(t);
        swap ^= kt;

        (x2, x3) = cswap(swap, x2, x3);
        (z2, z3) = cswap(swap, z2, z3);
        swap = kt;

        let a = (&x2 + &z2) % &p;
        let aa = &a.pow(2) % &p;
        let b = ((x2 + &p) - z2) % &p;
        let bb = &b.pow(2) % &p;
        let e = ((&p + &aa) - &bb) % &p;
        let c = (&x3 + &z3) % &p;
        let d = ((x3 + &p) - &z3) % &p;
        let da = (d * a) % &p;
        let cb = (c * b) % &p;
        x3 = ((&da + &cb).pow(2)) % &p;
        z3 = (x1 * ((da + &p) - &cb).pow(2)) % &p;
        x2 = (bb * &aa) % &p;
        z2 = ((aa + &a24 * &e) * &e) % &p;
    }

    (x2, _) = cswap(swap, x2, x3);
    (z2, _) = cswap(swap, z2, z3);
    (x2 * (z2.modpow(&(&p - 2.to_biguint().unwrap()), &p))) % p
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_cswap() {
        let a = BigUint::from_str(
            "31029842492115040904895560451863089656472772604678260265531221036453811406496",
        )
        .unwrap();
        let b = BigUint::from_str(
            "34426434033919594451155107781188821651316167215306631574996226621102155684838",
        )
        .unwrap();

        let (c, d) = cswap(true, a.clone(), b.clone());
        let (e, f) = cswap(false, a.clone(), b.clone());

        assert_eq!(a, d);
        assert_eq!(b, c);
        assert_eq!(a, e);
        assert_eq!(b, f);
    }

    #[test]
    fn test_single_1() {
        let k = [
            0xa5, 0x46, 0xe3, 0x6b, 0xf0, 0x52, 0x7c, 0x9d, 0x3b, 0x16, 0x15, 0x4b, 0x82, 0x46,
            0x5e, 0xdd, 0x62, 0x14, 0x4c, 0x0a, 0xc1, 0xfc, 0x5a, 0x18, 0x50, 0x6a, 0x22, 0x44,
            0xba, 0x44, 0x9a, 0xc4,
        ]
        .to_vec();
        let k = decode_scalar(k);

        assert_eq!(
            format!("{k}"),
            "31029842492115040904895560451863089656472772604678260265531221036453811406496"
        );

        let u = [
            0xe6, 0xdb, 0x68, 0x67, 0x58, 0x30, 0x30, 0xdb, 0x35, 0x94, 0xc1, 0xa4, 0x24, 0xb1,
            0x5f, 0x7c, 0x72, 0x66, 0x24, 0xec, 0x26, 0xb3, 0x35, 0x3b, 0x10, 0xa9, 0x03, 0xa6,
            0xd0, 0xab, 0x1c, 0x4c,
        ]
        .to_vec();
        let u = decode_u_coordinate(u);

        assert_eq!(
            format!("{u}"),
            "34426434033919594451155107781188821651316167215306631574996226621102155684838"
        );

        let u = x25519(&k, &u);
        let u = encode_u_coordinate(u);

        assert_eq!(
            u,
            [
                0xc3, 0xda, 0x55, 0x37, 0x9d, 0xe9, 0xc6, 0x90, 0x8e, 0x94, 0xea, 0x4d, 0xf2, 0x8d,
                0x08, 0x4f, 0x32, 0xec, 0xcf, 0x03, 0x49, 0x1c, 0x71, 0xf7, 0x54, 0xb4, 0x07, 0x55,
                0x77, 0xa2, 0x85, 0x52
            ]
        );
    }

    #[test]
    fn test_single_2() {
        let k = [
            0x4b, 0x66, 0xe9, 0xd4, 0xd1, 0xb4, 0x67, 0x3c, 0x5a, 0xd2, 0x26, 0x91, 0x95, 0x7d,
            0x6a, 0xf5, 0xc1, 0x1b, 0x64, 0x21, 0xe0, 0xea, 0x01, 0xd4, 0x2c, 0xa4, 0x16, 0x9e,
            0x79, 0x18, 0xba, 0x0d,
        ]
        .to_vec();
        let k = decode_scalar(k);

        assert_eq!(
            format!("{k}"),
            "35156891815674817266734212754503633747128614016119564763269015315466259359304"
        );

        let u = [
            0xe5, 0x21, 0x0f, 0x12, 0x78, 0x68, 0x11, 0xd3, 0xf4, 0xb7, 0x95, 0x9d, 0x05, 0x38,
            0xae, 0x2c, 0x31, 0xdb, 0xe7, 0x10, 0x6f, 0xc0, 0x3c, 0x3e, 0xfc, 0x4c, 0xd5, 0x49,
            0xc7, 0x15, 0xa4, 0x93,
        ]
        .to_vec();
        let u = decode_u_coordinate(u);

        assert_eq!(
            format!("{u}"),
            "8883857351183929894090759386610649319417338800022198945255395922347792736741"
        );

        let u = x25519(&k, &u);
        let u = encode_u_coordinate(u);

        assert_eq!(
            u,
            [
                0x95, 0xcb, 0xde, 0x94, 0x76, 0xe8, 0x90, 0x7d, 0x7a, 0xad, 0xe4, 0x5c, 0xb4, 0xb8,
                0x73, 0xf8, 0x8b, 0x59, 0x5a, 0x68, 0x79, 0x9f, 0xa1, 0x52, 0xe6, 0xf8, 0xf7, 0x64,
                0x7a, 0xac, 0x79, 0x57
            ]
        );
    }

    #[test]
    fn test_iterate() {
        let iv = [
            0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
        .to_vec();
        let mut k = decode_scalar(iv.clone());
        let mut u = decode_u_coordinate(iv);

        (k, u) = (x25519(&k, &u), k);

        assert_eq!(
            k.to_bytes_le(),
            [
                0x42, 0x2c, 0x8e, 0x7a, 0x62, 0x27, 0xd7, 0xbc, 0xa1, 0x35, 0x0b, 0x3e, 0x2b, 0xb7,
                0x27, 0x9f, 0x78, 0x97, 0xb8, 0x7b, 0xb6, 0x85, 0x4b, 0x78, 0x3c, 0x60, 0xe8, 0x03,
                0x11, 0xae, 0x30, 0x79
            ]
        );

        for _ in 1..1_000 {
            (k, u) = (x25519(&k, &u), k);
            println!("{:x?}", k.to_bytes_le());
        }

        println!("{:x?}", k.to_bytes_le());
        println!("{:x?}", u.to_bytes_le());
    }
}
