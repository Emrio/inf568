use crate::frodo640::{D, M_BAR, N, N_BAR};

pub const LEN_B: usize = D * N * N_BAR / 8;
pub const SIZE_MATRIX: usize = 2 * N * N_BAR;
pub const SIZE_C1: usize = D * M_BAR * N / 8;
pub const SIZE_C2: usize = D * M_BAR * N_BAR / 8;
