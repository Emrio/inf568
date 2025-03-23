// fn tchi (z: Element, lenchi: usize) -> Element {
//   (1 << (lenchi - 1)) - 1
// }

// pub fn sample(r: Vec<bool>, s: u32) -> Element {
//     assert!(r.len() != 0);

//     let r0 = r[0];
//     let r = &r[..r.len()];
//     let t = bits_to_element(r);

//     let mut e = 0;

//     for z in 0..s {
//       if t >
//     }

//     e
// }

// r = r15 | r14 | .. | r2 | r1 | r0
pub fn sample(r: u16, tchi: &[u16]) -> i16 {
    // assert!(r.len() != 0);

    let s = tchi.len();
    let r0 = r & 1;
    let t: u16 = r >> 1;

    let mut e = 0;
    for tchiz in &tchi[0..s] {
        e += (t > *tchiz) as i16;
    }

    let sign = (-1i16).pow(r0 as u32);
    e * sign
}

pub fn sample_matrix<const N1: usize, const N2: usize>(
    r: Vec<u16>,
    tchi: &[u16],
) -> [[i16; N2]; N1] {
    assert!(r.len() == N1 * N2);

    r.iter()
        .map(|r| sample(*r, tchi))
        .collect::<Vec<i16>>()
        .chunks_exact(N2)
        .map(|row| (*row).try_into().unwrap())
        .collect::<Vec<[i16; N2]>>()
        .try_into()
        .unwrap()
}
