#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    if S.last().unwrap() == &'r' {
        println!("er");
    } else {
        println!("ist")
    }
}
