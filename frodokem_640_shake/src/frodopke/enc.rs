use crate::frodo;
use crate::frodo640::{B, LEN_CHI, LEN_SEED_SE, M_BAR, N, N_BAR, Q, T_CHI};
use crate::shake128;
use crate::utils::matrix::{Matrix, matrix_add_modq, matrix_modq, matrix_multiplication_modq};
use crate::utils::random::random_bytes;
use crate::utils::sampling::prepare_r;

use super::PublicKey;

pub type Ciphertext = (Matrix<u16, M_BAR, N>, Matrix<u16, M_BAR, N_BAR>);

pub fn enc(message: Vec<bool>, public_key: PublicKey) -> Ciphertext {
    let (seed_a, matrix_b1) = public_key;
    let matrix_a = frodo::generate(&seed_a, Q);

    let seed_se = {
        let mut buffer = random_bytes::<{ 1 + LEN_SEED_SE }>();
        buffer[0] = 0x96;
        buffer
    };

    let r_s = M_BAR * N;
    let r_e1 = M_BAR * N;
    let r_e2 = M_BAR * N_BAR;

    let bitstream_size = (r_s + r_e1 + r_e2) * LEN_CHI;
    let bitstream = shake128::from(&seed_se, 2 * bitstream_size);

    let r = prepare_r(&bitstream[..r_s]);
    let matrix_s = frodo::sample_matrix(r, &T_CHI);

    let r = prepare_r(&bitstream[r_s..r_s + r_e1]);
    let matrix_e1 = frodo::sample_matrix(r, &T_CHI);

    let r = prepare_r(&bitstream[r_s + r_e1..]);
    let matrix_e2 = frodo::sample_matrix(r, &T_CHI);

    let matrix_s = matrix_modq(matrix_s, Q);
    let matrix_b2 = matrix_add_modq(
        matrix_multiplication_modq(matrix_s, matrix_a, Q),
        matrix_modq(matrix_e1, Q),
        Q,
    );
    let matrix_v = matrix_add_modq(
        matrix_add_modq(
            matrix_multiplication_modq(matrix_s, matrix_b1, Q),
            matrix_modq(matrix_e2, Q),
            Q,
        ),
        frodo::encode::<B, M_BAR, N_BAR>(message, Q),
        Q,
    );

    (matrix_b2, matrix_v)
}
