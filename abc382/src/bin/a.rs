#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (_, D): (usize, usize),
        S: Chars,
    }

    let ans = S.iter().filter(|x| **x == '.').count() + D;
    println!("{ans}");
}
