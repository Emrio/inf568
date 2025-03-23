pub fn bits_to_byte(bits: &[bool]) -> u8 {
    bits.iter()
        .enumerate()
        .map(|(i, x)| (*x as u8).wrapping_shr(i as u32))
        .sum()
}

pub fn u16_to_bits(mut x: u16, b: usize) -> Vec<bool> {
    let mut bits = Vec::with_capacity(b);
    bits.resize(b, false);

    for i in 0..b {
        bits[i] = x & 1 != 0;
        x >>= 1;
    }

    bits
}

pub fn bits_to_bytes<const N: usize>(bitstream: Vec<bool>) -> [u8; N] {
    assert!(bitstream.len() % 8 == 0);

    bitstream
        .chunks_exact(8)
        .map(bits_to_byte)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

pub fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    bytes
        .iter()
        .map(|byte| u16_to_bits(*byte as u16, 8))
        .collect::<Vec<Vec<bool>>>()
        .concat()
}
