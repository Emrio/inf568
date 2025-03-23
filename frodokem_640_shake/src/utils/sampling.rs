pub fn prepare_r(bitstream: &[u8]) -> Vec<u16> {
    bitstream
        .chunks_exact(2)
        .map(|x| u16::from_le_bytes(x.try_into().unwrap()))
        .collect::<Vec<u16>>()
}
