use crate::utils::bit_array::{bits_to_byte, u16_to_bits};

use super::types::Element;

fn ec<const B: usize>(k: Element, q: u16) -> Element {
    (k * q) >> B
}

fn dc<const B: usize>(c: Element, q: u16) -> Element {
    // round(x) = floor(x + 1/2)
    // round(a/b) = floor(a/b + 1/2) = floor((2a + b)/2b)
    (((c << (B + 1)) + q) / q) / 2
}

pub fn encode<const B: usize, const M_BAR: usize, const N_BAR: usize>(
    bit_stream: Vec<bool>,
    q: u16,
) -> [[Element; N_BAR]; M_BAR] {
    assert_eq!(bit_stream.len(), B * M_BAR * N_BAR);

    bit_stream
        .chunks_exact(B)
        .map(|x| bits_to_byte(x) as Element)
        .map(|k| ec::<B>(k, q))
        .collect::<Vec<Element>>()
        .chunks_exact(N_BAR)
        .map(|row| (*row).try_into().unwrap())
        .collect::<Vec<[Element; N_BAR]>>()
        .try_into()
        .unwrap()
}

pub fn decode<const B: usize, const M_BAR: usize, const N_BAR: usize>(
    matrix: [[Element; N_BAR]; M_BAR],
    q: u16,
) -> Vec<bool> {
    let bit_stream = matrix
        .iter()
        .flatten()
        .map(|c| dc::<B>(*c, q))
        .map(|c| u16_to_bits(c, B))
        .flatten()
        .collect::<Vec<bool>>();

    assert_eq!(bit_stream.len(), B * M_BAR * N_BAR);

    bit_stream
}
