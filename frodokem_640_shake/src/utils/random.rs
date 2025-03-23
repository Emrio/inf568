use rand::Rng;

pub fn random_bytes<const N: usize>() -> [u8; N] {
    rand::rng().random::<[u8; N]>()
}
