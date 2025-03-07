use crate::keccack;
use crate::sponge;
use crate::state::State;
use std::io::Read;

const R: usize = 168;

fn pad(data: &mut Vec<u8>) {
    assert!(data.len() < R);
    let pad_start = data.len();
    data.resize(R, 0);
    data[pad_start] = 0x1f;
    data[R - 1] += 0x80;
}

fn read_input_block_padded() -> (Vec<u8>, bool) {
    let mut input_data = Vec::with_capacity(R);
    let input_size = std::io::stdin()
        .take(R as u64)
        .read_to_end(&mut input_data)
        .unwrap();

    if input_size < R {
        pad(&mut input_data);
    }

    (input_data, input_size < R)
}

pub fn from_stdin(d: usize) -> Vec<u8> {
    let mut state = State::new();
    loop {
        let (input_data, reached_eof) = read_input_block_padded();
        state = sponge::absorb(R, keccack::keccak_p, state, &input_data[..]);

        if reached_eof {
            break;
        }
    }

    sponge::squeeze(R, keccack::keccak_p, d, state)
}

pub fn from(data: &[u8], d: usize) -> Vec<u8> {
    let sep = (data.len() / R) * R;
    let (begin, end) = data.split_at(sep);

    let state = State::new();
    let state = sponge::absorb(R, keccack::keccak_p, state, begin);

    let mut end = end.to_vec();
    pad(&mut end);

    let state = sponge::absorb(R, keccack::keccak_p, state, &end[..]);

    sponge::squeeze(R, keccack::keccak_p, d, state)
}
