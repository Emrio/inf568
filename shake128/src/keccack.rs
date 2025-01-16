mod chi;
mod iota;
mod pi;
mod rho;
mod theta;

use crate::state::State;

fn round(state: State, ir: usize) -> State {
    iota::iota(chi::chi(pi::pi(rho::rho(theta::theta(state)))), ir)

    // let state = theta(state);
    // println!("after theta:");
    // state.print();
    // let state = rho(state);
    // println!("after rho:");
    // state.print();
    // let state = pi(state);
    // println!("after pi:");
    // state.print();
    // let state = chi(state);
    // println!("after chi:");
    // state.print();
    // let state = iota(state, 0);
    // println!("after iota:");
    // state.print();
    // state
}

const NR: usize = 24;
const L: usize = 6;

pub fn keccak_p(state: State) -> State {
    let mut new_state = state.copy();
    for ir in (12 + 2 * L - NR)..(12 + 2 * L) {
        new_state = round(new_state, ir);
    }
    new_state
}
