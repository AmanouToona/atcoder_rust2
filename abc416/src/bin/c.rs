#![allow(non_snake_case)]
use itertools::{repeat_n, Itertools};
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (N, K, X) : (usize, usize, usize),
        S: [Chars; N],
    }

    let mut set: Vec<String> = Vec::new();
    for i in repeat_n(S, K).multi_cartesian_product() {
        set.push(i.iter().map(|x| x.iter().join("")).join(""));
    }
    set.sort();
    println!("{}", set[X - 1]);
}
