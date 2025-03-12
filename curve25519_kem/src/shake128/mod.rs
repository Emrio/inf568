mod keccack;
mod sponge;
mod state;

use state::State;

const R: usize = 168;

fn pad(data: &mut Vec<u8>) {
    assert!(data.len() < R);
    let pad_start = data.len();
    data.resize(R, 0);
    data[pad_start] = 0x1f;
    data[R - 1] += 0x80;
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

#[cfg(test)]
mod tests;
