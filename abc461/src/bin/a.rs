#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (A, D): (usize, usize),
    }

    if A <= D {
        println!("Yes");
    } else {
        println!("No");
    }
}
