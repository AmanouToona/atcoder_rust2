#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (L, R): (usize, usize)
    }

    println!("{}", R - L + 1);
}
