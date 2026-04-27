#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main() {
    input! {
        S: Chars,
        K: usize,
    }

    let mut s = HashSet::new();
    for i in S.iter().permutations(S.len()) {
        s.insert(i.iter().join(""));
    }

    let mut s: Vec<String> = Vec::from_iter(s);

    s.sort();

    println!("{}", s[K - 1]);
}
