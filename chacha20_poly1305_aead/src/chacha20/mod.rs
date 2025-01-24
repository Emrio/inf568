use std::{array, num::Wrapping};

fn quarter_round(a: u32, b: u32, c: u32, d: u32) -> (u32, u32, u32, u32) {
    let (mut a, mut b, mut c, mut d) = (Wrapping(a), Wrapping(b), Wrapping(c), Wrapping(d));
    a += b;
    d ^= a;
    d = Wrapping(d.0.rotate_left(16));
    c += d;
    b ^= c;
    b = Wrapping(b.0.rotate_left(12));
    a += b;
    d ^= a;
    d = Wrapping(d.0.rotate_left(8));
    c += d;
    b ^= c;
    b = Wrapping(b.0.rotate_left(7));
    (a.0, b.0, c.0, d.0)
}

#[derive(Clone)]
struct State([u32; 16]);

impl State {
    fn from(key: [u8; 32], nonce: [u8; 12], counter: u32) -> Self {
        Self([
            0x61707865,
            0x3320646e,
            0x79622d32,
            0x6b206574,
            u32::from_le_bytes(key[0..4].try_into().unwrap()),
            u32::from_le_bytes(key[4..8].try_into().unwrap()),
            u32::from_le_bytes(key[8..12].try_into().unwrap()),
            u32::from_le_bytes(key[12..16].try_into().unwrap()),
            u32::from_le_bytes(key[16..20].try_into().unwrap()),
            u32::from_le_bytes(key[20..24].try_into().unwrap()),
            u32::from_le_bytes(key[24..28].try_into().unwrap()),
            u32::from_le_bytes(key[28..32].try_into().unwrap()),
            counter,
            u32::from_le_bytes(nonce[0..4].try_into().unwrap()),
            u32::from_le_bytes(nonce[4..8].try_into().unwrap()),
            u32::from_le_bytes(nonce[8..12].try_into().unwrap()),
        ])
    }

    fn quarter_round(&mut self, (ia, ib, ic, id): (usize, usize, usize, usize)) {
        (self.0[ia], self.0[ib], self.0[ic], self.0[id]) =
            quarter_round(self.0[ia], self.0[ib], self.0[ic], self.0[id]);
    }

    fn column_round(&mut self) {
        self.quarter_round((0, 4, 8, 12));
        self.quarter_round((1, 5, 9, 13));
        self.quarter_round((2, 6, 10, 14));
        self.quarter_round((3, 7, 11, 15));
    }

    fn diagonal_round(&mut self) {
        self.quarter_round((0, 5, 10, 15));
        self.quarter_round((1, 6, 11, 12));
        self.quarter_round((2, 7, 8, 13));
        self.quarter_round((3, 4, 9, 14));
    }

    fn run_rounds(&mut self) {
        for _ in 0..10 {
            self.column_round();
            self.diagonal_round();
        }
    }

    fn to_buffer(self) -> [u8; 64] {
        let bytes: Vec<[u8; 4]> = self.0.iter().into_iter().map(|x| x.to_le_bytes()).collect();
        array::from_fn(|i| bytes[i >> 2][i & 3])
    }
}

impl std::ops::Add for State {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut state = [0u32; 16];
        for i in 0..16 {
            state[i] = (Wrapping(self.0[i]) + Wrapping(other.0[i])).0;
        }
        Self(state)
    }
}

fn chacha20_block(key: [u8; 32], nonce: [u8; 12], counter: u32) -> [u8; 64] {
    let initial_state = State::from(key, nonce, counter);
    let mut state = initial_state.clone();
    state.run_rounds();
    (state + initial_state).to_buffer()
}

pub fn chacha20_encrypt(key: [u8; 32], nonce: [u8; 12], counter: u32, plaintext: &[u8]) -> Vec<u8> {
    plaintext
        .chunks(64)
        .enumerate()
        .map(|(j, block)| {
            let key_stream: &[u8] = &chacha20_block(key, nonce, counter + j as u32)[..block.len()];
            block
                .iter()
                .zip(key_stream)
                .map(|(block_byte, stream_byte)| block_byte ^ stream_byte)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
        .concat()
}

#[cfg(test)]
mod tests;
