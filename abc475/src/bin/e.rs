#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (N, M, K): (usize, usize, usize),
        T: Chars,
        S: [Chars; N],
        Q: usize,
        query: [(usize, usize); Q],
    }
}
