#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
        N: usize,
    }

    let ans: String = S.iter().skip(N).take(S.len() - 2 * N).join("");
    println!("{ans}");
}
