#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N],
        Q: usize,
        X: [usize; Q],
    }

    let mut X: Vec<(usize, usize)> = X.iter().enumerate().map(|x| (*x.1, x.0)).collect();
    X.sort();
    let mut ans = vec![0; Q];
}
