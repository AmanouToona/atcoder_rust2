#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        X: Chars,
        Y: Chars,
        Q: usize,
    }

    let mut lrc = Vec::new();

    for q in 0..Q {
        input! {
            (L, R, C):(usize, usize, char),
        }
        lrc.push((L, R, C, q));
    }

    lrc.sort();
}
