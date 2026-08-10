#![allow(non_snake_case)]
use std::println;

use proconio::input;
fn main() {
    input! {
        N: usize,
        K: usize
    }

    println!("{}", N - K + 1);
}
