#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        X: usize
    }
    for i in 1..=3 {
        if i != X {
            println!("{i}");
            return;
        }
    }
}
