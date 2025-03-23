use crate::shake128;

use super::types::Element;

pub fn generate<const N: usize>(seed: &[u8], q: u16) -> [[Element; N]; N] {
    let mut matrix = [[0; N]; N];

    let mut buffer = {
        let mut buffer = Vec::with_capacity(seed.len() + 2);
        buffer.resize(seed.len() + 2, 0);
        buffer[2..].copy_from_slice(&seed);
        buffer
    };

    for i in 0..N {
        let counter_buffer = (i as u16).to_le_bytes();
        buffer[0..2].copy_from_slice(&counter_buffer);

        let output = shake128::from(&buffer, 2 * N);
        let row = output
            .chunks_exact(2)
            .map(|x| u16::from_le_bytes(x.try_into().unwrap()) % q)
            .collect::<Vec<u16>>();

        matrix[i].copy_from_slice(&row);
    }

    matrix
}
