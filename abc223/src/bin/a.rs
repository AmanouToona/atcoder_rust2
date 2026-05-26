#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        X: usize
    }
    if X > 100 && X % 100 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
