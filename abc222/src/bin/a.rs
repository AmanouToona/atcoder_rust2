#![allow(non_snake_case)]
use std::iter;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: Chars
    }

    let mut ans: String = iter::repeat('0')
        .take(4 - N.len())
        .chain(N.into_iter())
        .join("");
    println!("{ans}");
}
