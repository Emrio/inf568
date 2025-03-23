use crate::frodo640::{D, LEN_PKH, LEN_S, N, N_BAR};
use crate::utils::bit_array::bits_to_bytes;
use crate::utils::hash::hash;
use crate::utils::random::random_bytes;
use crate::{frodo, frodopke};

use super::types::{PublicKey, SecretKey};

pub fn keygen() -> (PublicKey, SecretKey) {
    let ((seed_a, matrix_b), matrix_s_t) = frodopke::keygen();

    let s = random_bytes::<LEN_S>();

    let b = frodo::pack::<D, N, N_BAR>(matrix_b);
    let b = bits_to_bytes(b);

    let public_key = PublicKey(seed_a, b);
    let pkh = hash::<LEN_PKH>(&public_key.to_bytes());

    (public_key, SecretKey(s, seed_a, b, matrix_s_t, pkh))
}
