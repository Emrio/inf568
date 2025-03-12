pub fn hash<const N: usize>(data: &[u8]) -> [u8; N] {
    shake128::from(data, N).try_into().unwrap()
}
