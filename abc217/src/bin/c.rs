#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        N: usize,
        P: [usize; N]
    }

    let mut Q = vec![0; N];
    for (i, &p) in P.iter().enumerate() {
        Q[p - 1] = i + 1;
    }

    println!("{}", Q.iter().join(" "));
}
