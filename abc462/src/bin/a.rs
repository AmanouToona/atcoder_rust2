#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let S: String = S.iter().filter(|x| x.is_digit(10)).join("");
    println!("{S}");
}
