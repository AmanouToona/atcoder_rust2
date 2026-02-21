#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
    }

    let mut A: Vec<usize> = (1..=N).collect();
    let mut base = 0;
    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                input! {p: usize, x: usize}
                let p = p - 1;
                A[(base + p) % N] = x;
            }
            2 => {
                input! {p: usize}
                let p = p - 1;
                println!("{}", A[(base + p) % N]);
            }
            3 => {
                input! {k: usize}
                base = (base + k) % N;
            }
            _ => {
                panic!("{q}")
            }
        }
    }
}
