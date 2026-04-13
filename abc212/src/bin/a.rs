#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (A, B): (usize, usize)
    }
    let ans = if A == 0 {
        "Silver"
    } else if B == 0 {
        "Gold"
    } else {
        "Alloy"
    };
    println!("{ans}");
}
