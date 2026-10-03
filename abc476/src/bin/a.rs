#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    match S.last() {
        Some('e') => {
            println!("{}r", S.iter().join(""));
        }
        _ => {
            println!("{}er", S.iter().join(""));
        }
    }
}
