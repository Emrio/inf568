use crate::frodo;
use crate::frodo640::{B, M_BAR, N_BAR, Q};
use crate::utils::matrix::{matrix_multiplication_modq, matrix_sub_modq, matrix_transpose};

use super::{Ciphertext, SecretKey};

pub fn dec(ciphertext: Ciphertext, secret_key: SecretKey) -> Vec<bool> {
    let (c1, c2) = ciphertext;
    let matrix_s_t = secret_key;

    let matrix_s = matrix_transpose(matrix_s_t);
    let matrix_m = matrix_sub_modq(c2, matrix_multiplication_modq(c1, matrix_s, Q), Q);

    frodo::decode::<B, M_BAR, N_BAR>(matrix_m, Q)
}
