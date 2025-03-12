use super::super::state::State;
use super::L;

const fn rc(t: u8) -> u8 {
    if t % 255 == 0 {
        return 1;
    }

    let mut r = 0x01;
    let mut i = 0;
    while i < t {
        let r8 = r & 0x80;
        r <<= 1;
        if r8 != 0 {
            r ^= 0x71;
        }
        i += 1;
    }
    (r & 1 == 1) as u8
}

const RC: [u8; 255] = const {
    let mut _rc = [0u8; 255];
    let mut i = 0;
    while i < 255 {
        _rc[i] = rc(i as u8);
        i += 1;
    }
    _rc
};

const fn compute_rc(ir: usize) -> u64 {
    let mut rc = 0u64;
    let mut j = 0;
    while j <= L {
        rc |= (RC[7 * ir + j] as u64) << ((1 << j) - 1);
        j += 1;
    }
    rc
}

pub fn iota(state: State, ir: usize) -> State {
    let mut new_state = state.copy();
    new_state[[0, 0]] ^= compute_rc(ir);
    new_state
}
