#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let S: String = S.iter().join("");
    if S == "Hello,World!" {
        println!("AC")
    } else {
        println!("WA")
    }
}
