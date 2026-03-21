#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        N: usize,
    }

    let ans: String = (1..=N).rev().join(",");
    println!("{ans}");
    // let ans: String = (1..=N).iter().rev().join(",");
    // println!("{ans}");
}
