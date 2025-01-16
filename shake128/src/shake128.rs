use crate::keccack;
use crate::sponge;

pub fn from_stdin(d: usize) -> Vec<u8> {
    let s = sponge::absorb_from_stdin(168, keccack::keccak_p);
    sponge::squeeze(168, keccack::keccak_p, d, s)
}
