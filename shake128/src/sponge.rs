// IMPORTANT NOTE: d and r are expressed in bytes, not bits contrary to the FIPS 202 standard.

use crate::state::State;

type F = fn(State) -> State;

fn absorb_fixed(r: usize, state: State, data: &[u8]) -> State {
    assert_eq!(data.len(), r);

    let mut new_state = state.copy();

    let mut data_index = 0;
    for i in 0..(r / 8) {
        let mut lane = 0u64;
        for j in 0..8 {
            lane |= (data[data_index] as u64) << (j * 8);
            data_index += 1;
        }
        let x = i % 5;
        let y = i / 5;
        new_state[[x, y]] ^= lane;
    }

    new_state
}

// Input is assumed to be a multiple of r bytes (padded if necessary)
pub fn absorb(r: usize, f: F, state: State, data: &[u8]) -> State {
    assert_eq!(data.len() % r, 0);

    let mut state = state;
    let mut data_index = 0;
    while data_index < data.len() {
        let chunk = &data[data_index..data_index + r];
        state = absorb_fixed(r, state, chunk);
        state = f(state);
        data_index += r;
    }
    state
}

pub fn squeeze(r: usize, f: F, d: usize, state: State) -> Vec<u8> {
    let mut output = Vec::with_capacity(d);
    let mut state = state;

    loop {
        for i in 0..r {
            output.push(state.get_byte(i));
            if output.len() == d {
                return output;
            }
        }
        state = f(state);
    }
}
