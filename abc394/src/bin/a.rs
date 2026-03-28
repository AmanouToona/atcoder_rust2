#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let ans: String = S.iter().filter(|&s| *s == '2').join("");
    println!("{ans}");
}
