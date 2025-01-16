use crate::state::State;

pub fn chi(state: State) -> State {
    let mut new_state = state.copy();
    for x in 0..5 {
        for y in 0..5 {
            new_state[[x, y]] ^= !state[[(x + 1) % 5, y]] & state[[(x + 2) % 5, y]];
        }
    }
    new_state
}
