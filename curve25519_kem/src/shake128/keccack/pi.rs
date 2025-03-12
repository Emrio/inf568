use super::super::state::State;

pub fn pi(state: State) -> State {
    let mut new_state = State::new();
    for x in 0..5 {
        for y in 0..5 {
            new_state[[x, y]] = state[[(x + 3 * y) % 5, x]];
        }
    }
    new_state
}
