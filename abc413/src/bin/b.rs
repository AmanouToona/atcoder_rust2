#![allow(non_snake_case)]
use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let mut set: HashSet<String> = HashSet::new();
    for (i, j) in iproduct!(0..N, 0..N) {
        if i == j {
            continue;
        }
        set.insert(S[i].iter().chain(S[j].iter()).join(""));
    }

    println!("{}", set.len());
}
