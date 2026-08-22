#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        mut S: Chars,
    }

    for c in S.iter_mut() {
        if *c != 'A' {
            *c = '.';
        }
    }

    let ans: String = S.iter().join("");
    println!("{ans}");
}
