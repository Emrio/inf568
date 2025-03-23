use super::super::state::State;

pub fn theta(state: State) -> State {
    let mut c = [0u64; 5];
    let mut new_state = state.copy();
    for x in 0..5 {
        c[x] = state[[x, 0]] ^ state[[x, 1]] ^ state[[x, 2]] ^ state[[x, 3]] ^ state[[x, 4]];
    }
    for x in 0..5 {
        let d = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        for y in 0..5 {
            new_state[[x, y]] ^= d;
        }
    }
    new_state
}
