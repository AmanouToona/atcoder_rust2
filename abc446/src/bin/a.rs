#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        mut S: Chars
    }

    let ans: String = "Of"
        .chars()
        .chain(S.iter().flat_map(|x| x.to_lowercase()))
        .join("");
    println!("{ans}");
}
