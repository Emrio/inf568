use crate::frodo640::{LEN_PKH, LEN_S, LEN_SEED_A, LEN_SS, N, N_BAR};
use crate::utils::matrix::Matrix;

use super::constants::{LEN_B, SIZE_C1, SIZE_C2, SIZE_MATRIX};

pub struct PublicKey(pub [u8; LEN_SEED_A], pub [u8; LEN_B]);
impl PublicKey {
    pub fn from_bytes(input: &[u8]) -> Self {
        assert_eq!(input.len(), LEN_SEED_A + LEN_B);

        Self(
            input[..LEN_SEED_A].to_owned().try_into().unwrap(),
            input[LEN_SEED_A..].to_owned().try_into().unwrap(),
        )
    }

    pub fn to_bytes(&self) -> [u8; LEN_SEED_A + LEN_B] {
        let mut buffer = Vec::with_capacity(LEN_SEED_A + LEN_B);
        buffer.extend_from_slice(&self.0);
        buffer.extend_from_slice(&self.1);

        buffer.try_into().unwrap()
    }
}

pub struct SecretKey(
    pub [u8; LEN_S],
    pub [u8; LEN_SEED_A],
    pub [u8; LEN_B],
    pub Matrix<u16, N_BAR, N>,
    pub [u8; LEN_PKH],
);
impl SecretKey {
    //     pub fn from_bytes(input: &[u8]) -> Self {
    //         assert_eq!(input.len(), 32 + LS + 32 + D);

    //         Self(
    //             input[..32].to_owned().try_into().unwrap(),
    //             input[32..32 + LS].to_owned().try_into().unwrap(),
    //             input[32 + LS..32 + LS + 32].to_owned().try_into().unwrap(),
    //             input[32 + LS + 32..].to_owned().try_into().unwrap(),
    //         )
    //     }

    pub fn to_bytes(self) -> [u8; LEN_S + LEN_SEED_A + LEN_B + SIZE_MATRIX + LEN_PKH] {
        let matrix_buffer = self
            .3
            .iter()
            .flatten()
            .flat_map(|x| x.to_le_bytes())
            .collect::<Vec<u8>>();

        let mut buffer = Vec::with_capacity(LEN_S + LEN_SEED_A + LEN_B + SIZE_MATRIX + LEN_PKH);
        buffer.extend_from_slice(&self.0);
        buffer.extend_from_slice(&self.1);
        buffer.extend_from_slice(&self.2);
        buffer.extend_from_slice(&matrix_buffer);
        buffer.extend_from_slice(&self.4);

        buffer.try_into().unwrap()
    }
}

pub struct CipherText(pub [u8; SIZE_C1], pub [u8; SIZE_C2]);
impl CipherText {
    //     pub fn from_bytes(input: &[u8]) -> Self {
    //         assert_eq!(input.len(), 32 + LS + 32 + D);

    //         Self(
    //             input[..32].to_owned().try_into().unwrap(),
    //             input[32..32 + LS].to_owned().try_into().unwrap(),
    //             input[32 + LS..32 + LS + 32].to_owned().try_into().unwrap(),
    //             input[32 + LS + 32..].to_owned().try_into().unwrap(),
    //         )
    //     }

    pub fn to_bytes(self) -> [u8; SIZE_C1 + SIZE_C2] {
        let mut buffer = Vec::with_capacity(SIZE_C1 + SIZE_C2);
        buffer.extend_from_slice(&self.0);
        buffer.extend_from_slice(&self.1);

        buffer.try_into().unwrap()
    }
}

pub type SharedSecret = [u8; LEN_SS];
