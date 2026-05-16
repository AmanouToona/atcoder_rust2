#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        X: usize,
    }
    println!("{}", A[X - 1]);
}
