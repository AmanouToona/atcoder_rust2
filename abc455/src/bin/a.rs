#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (A, B, C): (usize, usize, usize)
    }

    if A != B && B == C {
        println!("Yes");
    } else {
        println!("No");
    }
}
