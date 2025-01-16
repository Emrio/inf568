use crate::state::{xy_to_arr, State};

const OFFSETS: [u32; 25] = const {
    let mut offsets = [0u32; 25];

    let mut t = 0;
    let mut x = 1;
    let mut y = 0;

    while t < 24 {
        offsets[xy_to_arr(x, y)] = (t + 1) * (t + 2) / 2;
        t += 1;
        [x, y] = [y, (2 * x + 3 * y) % 5];
    }

    offsets
};

pub fn rho(state: State) -> State {
    let mut new_state = State::new();
    for x in 0..5 {
        for y in 0..5 {
            let offset = OFFSETS[xy_to_arr(x, y)];
            new_state[[x, y]] = state[[x, y]].rotate_left(offset);
        }
    }
    new_state
}
