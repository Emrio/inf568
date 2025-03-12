use std::{
    fmt,
    ops::{Index, IndexMut},
};

pub const fn xy_to_arr(x: usize, y: usize) -> usize {
    assert!(x < 5 && y < 5);

    // let [x, y] = [(x + 2) % 5, (5 - y + 2) % 5];
    return y * 5 + x;
}

pub struct State {
    a: [u64; 25],
}

impl State {
    pub fn new() -> State {
        State { a: [0u64; 25] }
    }

    pub fn copy(&self) -> State {
        State { a: self.a }
    }

    pub const fn get_byte(&self, i: usize) -> u8 {
        let lane = self.a[i / 8];
        (lane >> (i % 8) * 8) as u8
    }
}

impl Index<[usize; 2]> for State {
    type Output = u64;

    fn index(&self, index: [usize; 2]) -> &u64 {
        &self.a[xy_to_arr(index[0], index[1])]
    }
}

impl IndexMut<[usize; 2]> for State {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut u64 {
        &mut self.a[xy_to_arr(index[0], index[1])]
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                write!(f, "{:016x} ", self.a[xy_to_arr(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
