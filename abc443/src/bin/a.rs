#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars
    }

    let ans: String = S.iter().chain(['s'].iter()).join("");
    println!("{ans}");
}
