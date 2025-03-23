use crate::utils::bit_array::{bits_to_byte, u16_to_bits};

use super::types::Element;

pub fn pack<const D: usize, const N1: usize, const N2: usize>(
    matrix: [[Element; N2]; N1],
) -> Vec<bool> {
    let bit_stream = matrix
        .iter()
        .flatten()
        .map(|c| u16_to_bits(*c, D))
        .flatten()
        .collect::<Vec<bool>>();

    assert_eq!(bit_stream.len(), D * N1 * N2);

    bit_stream
}

pub fn unpack<const D: usize, const N1: usize, const N2: usize>(
    bit_stream: Vec<bool>,
) -> [[Element; N2]; N1] {
    assert_eq!(bit_stream.len(), D * N1 * N2);

    bit_stream
        .chunks_exact(D)
        .map(|x| bits_to_byte(x) as Element)
        .collect::<Vec<Element>>()
        .chunks_exact(N2)
        .map(|row| (*row).try_into().unwrap())
        .collect::<Vec<[Element; N2]>>()
        .try_into()
        .unwrap()
}
