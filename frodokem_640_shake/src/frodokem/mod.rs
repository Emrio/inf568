mod constants;
mod encaps;
mod keygen;
mod types;

// pub use decaps::*;
pub use encaps::*;
pub use keygen::*;
pub use types::*;

// pub fn decaps(ciphertext: CipherText, secret_key: SecretKey) -> EncryptionKey {
//     let SecretKey(sk, s, pk, pkh) = secret_key;

//     let m = pke::decrypt(&ciphertext, sk);

//     let rk = hash::<RK_SPACE>(&[pkh, m].concat());
//     let (r, k) = rk.split_at(32);

//     let k0 = hash::<ENCRYPTION_KEY_SIZE>(&[&ciphertext[..], k].concat());
//     let k1 = hash::<ENCRYPTION_KEY_SIZE>(&[&ciphertext[..], &s[..]].concat());

//     let ciphertext2 = pke::encrypt(&m, &pk, &r.try_into().unwrap());

//     let mask1 = !!ciphertext
//         .iter()
//         .zip(ciphertext2)
//         .map(|(c1, c2)| c1 ^ c2)
//         .reduce(|acc, cur| acc | cur)
//         .unwrap();
//     let mask0 = mask1.wrapping_sub(1);

//     k0.iter()
//         .zip(k1)
//         .map(|(k0, k1)| (mask0 & k0) | (mask1 & k1))
//         .collect::<Vec<u8>>()
//         .try_into()
//         .unwrap()
// }
