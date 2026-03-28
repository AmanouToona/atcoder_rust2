#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        mut S: [Chars; N],
    }

    S.sort_by_key(|x| x.len());
    let ans: String = S.iter().flatten().join("");
    println!("{ans}");
}
