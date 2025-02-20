use std::{cmp::Ordering, ops};

use num_bigint::BigUint;

use super::constants;

#[derive(Clone)]
pub struct Point {
    x: BigUint,
    y: BigUint,
    z: BigUint,
    t: BigUint,
}

impl Point {
    pub fn new(x: BigUint, y: BigUint) -> Self {
        let p = constants::p();
        Self {
            t: (x.clone() * &y) % &p,
            z: BigUint::from(1u32),
            x: x % &p,
            y: y % &p,
        }
    }

    pub fn zero() -> Point {
        Self {
            x: BigUint::ZERO,
            y: BigUint::from(1u32),
            z: BigUint::from(1u32),
            t: BigUint::ZERO,
        }
    }

    pub fn double(self) -> Self {
        let p = &constants::p();
        let two = BigUint::from(2u32);

        let a = self.x.modpow(&two, p);
        let b = self.y.modpow(&two, p);
        let c = self.z.modpow(&two, p) * &two;
        let h = &a + &b;
        let e = sub(&h, &(self.x + self.y).modpow(&two, p), p);
        let g = sub(&a, &b, p);
        let f = c + &g;

        let x = (&e * &f) % p;
        let t = (e * &h) % p;
        let y = (h * &g) % p;
        let z = (f * g) % p;

        Self { x, y, z, t }
    }

    pub fn to_bytes(self) -> [u8; 32] {
        let p = &constants::p();
        let zinv = self.z.modinv(p).unwrap();
        let x = (self.x * &zinv) % p;
        let y = (self.y * &zinv) % p;
        let x = (x & BigUint::from(1u32)) << 255;
        let xy: BigUint = y | x;
        let mut result = xy.to_bytes_le();
        result.resize(32, 0);
        result.try_into().unwrap()
    }

    pub fn from_bytes(input: [u8; 32]) -> Result<Self, &'static str> {
        let mut y = BigUint::from_bytes_le(&input);
        let sign = y.bit(255);
        y.set_bit(255, false);

        let p = &constants::p();
        let x = recover_x(&y, sign, p)?;

        Ok(Self {
            z: BigUint::from(1u32),
            t: (&x * &y) % p,
            x,
            y,
        })
    }
}

fn recover_x(y: &BigUint, sign: bool, p: &BigUint) -> Result<BigUint, &'static str> {
    if y.cmp(p) != Ordering::Less {
        return Err("could not recover x : y >= p");
    }

    let d = constants::d();
    let one = BigUint::from(1u32);

    let y2 = y * y;
    let x2 = ((d * &y2 + &one).modinv(p).unwrap() * (y2 - &one)) % p;

    if x2.cmp(&BigUint::ZERO) == Ordering::Equal {
        if sign {
            return Err("could not recover x : x2 == 0");
        }

        return Ok(BigUint::ZERO);
    }

    let mut x = x2.modpow(&((p + BigUint::from(3u32)) >> 3), p);

    if ((&x * &x) % p).cmp(&x2) != Ordering::Equal {
        x *= BigUint::from(2u32).modpow(&((p - &one) >> 2), p);
        x %= p;
    }
    if ((&x * &x) % p).cmp(&x2) != Ordering::Equal {
        return Err("could not recover x : sqrt(x2) failed");
    }

    if x.bit(0) != sign {
        x = p - x;
    }

    Ok(x)
}

fn sub(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    (a + p) - (b % p)
}

impl ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, v: &Point) -> Self::Output {
        let u = self;
        let two = BigUint::from(2u32);
        let p = &constants::p();

        let a = (sub(&u.y, &u.x, p) * sub(&v.y, &v.x, p)) % p;
        let b = ((&u.y + &u.x) * (&v.y + &v.x)) % p;
        let c = (((constants::d() * &u.t) * &two) * &v.t) % p;
        let d = ((two * &u.z) * &v.z) % p;
        let e = sub(&b, &a, p);
        let f = sub(&d, &c, p);
        let g = d + &c;
        let h = b + &a;

        let x = (&e * &f) % p;
        let z = (f * &g) % p;
        let y = (g * &h) % p;
        let t = (e * &h) % p;

        Point { x, y, z, t }
    }
}

impl ops::Mul<&BigUint> for Point {
    type Output = Point;

    fn mul(self, s: &BigUint) -> Self::Output {
        let mut p = self.clone();
        let mut q = Point::zero();

        for i in 0..s.bits() {
            if s.bit(i) {
                q = &q + &p;
            }
            p = p.double();
        }

        q
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let p = &constants::p();

        match (
            ((&self.x * &other.z) % p).cmp(&((&other.x * &self.z) % p)),
            ((&self.y * &other.z) % p).cmp(&((&other.y * &self.z) % p)),
        ) {
            (Ordering::Equal, Ordering::Equal) => true,
            _ => false,
        }
    }
}
