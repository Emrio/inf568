use crate::frodo640::{
    B, D, LEN_CHI, LEN_K, LEN_MU, LEN_PKH, LEN_SEED_SE, LEN_SS, M_BAR, N, N_BAR, Q, T_CHI,
};
use crate::utils::bit_array::{bits_to_bytes, bytes_to_bits};
use crate::utils::hash::hash;
use crate::utils::matrix::{matrix_add_modq, matrix_modq, matrix_multiplication_modq};
use crate::utils::random::random_bytes;
use crate::utils::sampling::prepare_r;
use crate::{frodo, shake128};

use super::constants::{SIZE_C1, SIZE_C2};
use super::types::{CipherText, PublicKey, SharedSecret};

pub fn encaps(public_key: PublicKey) -> (CipherText, SharedSecret) {
    let PublicKey(seed_a, b) = public_key;

    let message = random_bytes::<LEN_MU>();

    let pkh = hash::<LEN_PKH>(&public_key.to_bytes());

    let (seed_se, k) = {
        let mut buffer = Vec::with_capacity(LEN_PKH + LEN_MU);
        buffer.extend_from_slice(&pkh);
        buffer.extend_from_slice(&message);
        let output = shake128::from(&buffer, LEN_SEED_SE + LEN_K);

        let mut seed_se = [0u8; 1 + LEN_SEED_SE];
        seed_se[0] = 0x96;
        seed_se[1..].copy_from_slice(&output[..LEN_SEED_SE]);

        let k: [u8; LEN_K] = output[LEN_SEED_SE..].try_into().unwrap();

        (seed_se, k)
    };

    let r_s = M_BAR * N * LEN_CHI;
    let r_e1 = M_BAR * N * LEN_CHI;
    let r_e2 = M_BAR * N_BAR * LEN_CHI;

    let bitstream_size = r_s + r_e1 + r_e2;
    let bitstream = shake128::from(&seed_se, bitstream_size);

    let r = prepare_r(&bitstream[..r_s]);
    let matrix_s = frodo::sample_matrix::<M_BAR, N>(r, &T_CHI);
    let matrix_s = matrix_modq(matrix_s, Q);

    let r = prepare_r(&bitstream[r_s..r_s + r_e1]);
    let matrix_e1 = frodo::sample_matrix::<M_BAR, N>(r, &T_CHI);
    let matrix_e1 = matrix_modq(matrix_e1, Q);

    let matrix_a = frodo::generate(&seed_a, Q);
    let matrix_b1 = matrix_add_modq(
        matrix_multiplication_modq(matrix_s, matrix_a, Q),
        matrix_e1,
        Q,
    );
    let c1 = frodo::pack::<D, M_BAR, N>(matrix_b1);
    let c1 = bits_to_bytes::<SIZE_C1>(c1);

    let r = prepare_r(&bitstream[r_s + r_e1..]);
    let matrix_e2 = frodo::sample_matrix::<M_BAR, N_BAR>(r, &T_CHI);
    let matrix_e2 = matrix_modq(matrix_e2, Q);

    let matrix_b2 = frodo::unpack::<D, N, N_BAR>(bytes_to_bits(&b));

    let matrix_v = matrix_add_modq(
        matrix_multiplication_modq(matrix_s, matrix_b2, Q),
        matrix_e2,
        Q,
    );
    let matrix_c = matrix_add_modq(
        matrix_v,
        frodo::encode::<B, M_BAR, N_BAR>(bytes_to_bits(&message), Q),
        Q,
    );
    let c2 = frodo::pack::<D, M_BAR, N_BAR>(matrix_c);
    let c2 = bits_to_bytes::<SIZE_C2>(c2);

    let shared_secret = {
        let mut buffer = Vec::with_capacity(c1.len() + c2.len() + k.len());
        buffer.extend_from_slice(&c1);
        buffer.extend_from_slice(&c2);
        buffer.extend_from_slice(&k);

        hash::<LEN_SS>(&buffer)
    };

    (CipherText(c1, c2), shared_secret)
}
