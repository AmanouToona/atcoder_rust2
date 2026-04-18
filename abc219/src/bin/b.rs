#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
        T: Chars,
    }

    let T: Vec<usize> = T
        .iter()
        .map(|x| x.to_digit(10).unwrap() as usize - 1)
        .collect();

    let S = [s1, s2, s3];

    let ans: String = T.iter().flat_map(|t| S[*t].clone()).join("");
    println!("{ans}");
}
