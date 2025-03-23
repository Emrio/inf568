pub type Matrix<T, const M: usize, const N: usize> = [[T; N]; M];

pub fn matrix_multiplication_modq<const M: usize, const N: usize, const P: usize>(
    a: Matrix<u16, M, N>,
    b: Matrix<u16, N, P>,
    q: u16,
) -> Matrix<u16, M, P> {
    let mut c = [[0u16; P]; M];

    for i in 0..M {
        for j in 0..P {
            for k in 0..N {
                c[i][j] = (c[i][j] as u32 + (a[i][k] as u32 * b[k][j] as u32) % (q as u32)) as u16;
            }
            c[i][j] %= q;
        }
    }

    c
}

pub fn matrix_transpose<const M: usize, const N: usize>(a: Matrix<u16, M, N>) -> Matrix<u16, N, M> {
    let mut a_t = [[0u16; M]; N];

    for i in 0..N {
        for j in 0..M {
            a_t[i][j] = a[j][i];
        }
    }

    a_t
}

pub fn matrix_modq<const M: usize, const N: usize>(
    a: Matrix<i16, M, N>,
    q: u16,
) -> Matrix<u16, M, N> {
    let mut b = [[0u16; N]; M];
    let q = q as i16;

    for i in 0..M {
        for j in 0..N {
            b[i][j] = a[i][j].rem_euclid(q) as u16;
        }
    }

    b
}

pub fn matrix_add_modq<const M: usize, const N: usize>(
    a: Matrix<u16, M, N>,
    b: Matrix<u16, M, N>,
    q: u16,
) -> Matrix<u16, M, N> {
    let mut c = [[0u16; N]; M];

    for i in 0..M {
        for j in 0..N {
            c[i][j] = (a[i][j] as u32 + b[i][j] as u32 % (q as u32)) as u16;
        }
    }

    c
}

pub fn matrix_sub_modq<const M: usize, const N: usize>(
    a: Matrix<u16, M, N>,
    b: Matrix<u16, M, N>,
    q: u16,
) -> Matrix<u16, M, N> {
    let mut c = [[0u16; N]; M];

    for i in 0..M {
        for j in 0..N {
            c[i][j] = ((q + a[i][j]) - b[i][j]) % q;
        }
    }

    c
}
