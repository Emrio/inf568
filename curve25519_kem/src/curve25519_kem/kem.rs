use rand::Rng;

use super::hash::hash;
use super::pke::{self, CIPHERTEXT_SIZE};

const LS: usize = 256 / 8;
const D: usize = 32;

pub type PublicKey = [u8; 32];

pub struct SecretKey([u8; 32], [u8; LS], PublicKey, [u8; D]);
impl SecretKey {
    pub fn from_bytes(input: &[u8]) -> Self {
        assert_eq!(input.len(), 32 + LS + 32 + D);

        Self(
            input[..32].to_owned().try_into().unwrap(),
            input[32..32 + LS].to_owned().try_into().unwrap(),
            input[32 + LS..32 + LS + 32].to_owned().try_into().unwrap(),
            input[32 + LS + 32..].to_owned().try_into().unwrap(),
        )
    }

    pub fn to_bytes(self) -> [u8; 32 + LS + 32 + D] {
        [self.0, self.1, self.2, self.3]
            .concat()
            .try_into()
            .unwrap()
    }
}

pub fn keygen() -> (PublicKey, SecretKey) {
    let (pk, sk) = pke::keygen();
    let s = rand::rng().random::<[u8; LS]>();

    let pkh = hash::<D>(&pk);

    (pk, SecretKey(sk, s, pk, pkh))
}

const M_SPACE: usize = 256 / 8;
const R_SPACE: usize = 32;
const K_SPACE: usize = 32;
const RK_SPACE: usize = R_SPACE + K_SPACE;
const ENCRYPTION_KEY_SIZE: usize = 16;

pub type CipherText = [u8; CIPHERTEXT_SIZE];
pub type EncryptionKey = [u8; ENCRYPTION_KEY_SIZE];

pub fn encaps(pk: PublicKey) -> (CipherText, EncryptionKey) {
    let m = rand::rng().random::<[u8; M_SPACE]>();

    let pkh = hash::<D>(&pk);

    let rk = hash::<RK_SPACE>(&[&pkh[..], &m].concat());
    let (r, k) = {
        let (r, k) = rk.split_at(R_SPACE);
        (r.try_into().unwrap(), k)
    };

    let ciphertext = pke::encrypt(&m, &pk, &r);
    let encryption_key = {
        let mut buffer = [0u8; CIPHERTEXT_SIZE + K_SPACE];
        buffer[..CIPHERTEXT_SIZE].copy_from_slice(&ciphertext);
        buffer[CIPHERTEXT_SIZE..].copy_from_slice(k);
        hash::<ENCRYPTION_KEY_SIZE>(&buffer)
    };

    (ciphertext, encryption_key)
}

pub fn decaps(ciphertext: CipherText, secret_key: SecretKey) -> EncryptionKey {
    let SecretKey(sk, s, pk, pkh) = secret_key;

    let m = pke::decrypt(&ciphertext, sk);

    let rk = hash::<RK_SPACE>(&[pkh, m].concat());
    let (r, k) = rk.split_at(32);

    let k0 = hash::<ENCRYPTION_KEY_SIZE>(&[&ciphertext[..], k].concat());
    let k1 = hash::<ENCRYPTION_KEY_SIZE>(&[&ciphertext[..], &s[..]].concat());

    let ciphertext2 = pke::encrypt(&m, &pk, &r.try_into().unwrap());

    let mask1 = !!ciphertext
        .iter()
        .zip(ciphertext2)
        .map(|(c1, c2)| c1 ^ c2)
        .reduce(|acc, cur| acc | cur)
        .unwrap();
    let mask0 = mask1.wrapping_sub(1);

    k0.iter()
        .zip(k1)
        .map(|(k0, k1)| (mask0 & k0) | (mask1 & k1))
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}
