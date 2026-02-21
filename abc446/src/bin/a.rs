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
        .chain(S.iter().cloned().map(|x| {
            if x.is_uppercase() {
                x.to_ascii_lowercase()
            } else {
                x
            }
        }))
        .join("");
    println!("{ans}");
}
