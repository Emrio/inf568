// IMPORTANT NOTE: d and r are expressed in bytes, not bits contrary to the FIPS 202 standard.

use crate::state::State;
use std::io::Read;

type F = fn(State) -> State;

fn feed(r: usize, state: State, data: &[u8]) -> State {
    assert!(data.len() == r);

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

fn read_input_block_padded(r: usize) -> (Vec<u8>, bool) {
    let mut input_data = vec![0u8; r];
    let input_size = std::io::stdin()
        .take(r as u64)
        .read(&mut input_data)
        .unwrap();

    if input_size < r {
        input_data[input_size] = 0x1f;
        input_data[r - 1] += 0x80
    }

    (input_data, input_size < r)
}

pub fn absorb_from_stdin(r: usize, f: F) -> State {
    let mut state = State::new();
    loop {
        let (input_data, reached_eof) = read_input_block_padded(r);
        state = feed(r, state, &input_data[..]);
        state = f(state);

        if reached_eof {
            break;
        }
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
