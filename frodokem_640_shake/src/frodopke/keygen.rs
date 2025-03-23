use crate::frodo;
use crate::frodo640::{LEN_CHI, LEN_SEED_A, LEN_SEED_SE, N, N_BAR, Q, T_CHI};
use crate::shake128;
use crate::utils::matrix::{
    Matrix, matrix_add_modq, matrix_modq, matrix_multiplication_modq, matrix_transpose,
};
use crate::utils::random::random_bytes;
use crate::utils::sampling::prepare_r;

pub type PublicKey = ([u8; LEN_SEED_A], Matrix<u16, N, N_BAR>);
pub type SecretKey = Matrix<u16, N_BAR, N>;

pub fn keygen() -> (PublicKey, SecretKey) {
    let seed_a = random_bytes::<LEN_SEED_A>();
    let matrix_a = frodo::generate::<N>(&seed_a, Q);

    let seed_se = {
        let mut buffer = random_bytes::<{ 1 + LEN_SEED_SE }>();
        buffer[0] = 0x5F;
        buffer
    };

    let bitstream_size = N * N_BAR * LEN_CHI;
    let bitstream = shake128::from(&seed_se, 2 * bitstream_size);

    let r = prepare_r(&bitstream[..bitstream_size]);
    let matrix_s_t = frodo::sample_matrix::<N_BAR, N>(r, &T_CHI);

    let r = prepare_r(&bitstream[bitstream_size..]);
    let matrix_e = frodo::sample_matrix::<N, N_BAR>(r, &T_CHI);

    let matrix_s_t = matrix_modq(matrix_s_t, Q);

    let matrix_s = matrix_transpose(matrix_s_t);
    let matrix_b = matrix_add_modq(
        matrix_multiplication_modq(matrix_a, matrix_s, Q),
        matrix_modq(matrix_e, Q),
        Q,
    );

    let public_key = (seed_a, matrix_b);
    let secret_key = matrix_s_t;

    (public_key, secret_key)
}
