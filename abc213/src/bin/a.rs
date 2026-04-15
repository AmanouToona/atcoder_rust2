#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (A, B): (usize, usize),
    }

    for i in 0..=255 {
        if A ^ i == B {
            println!("{i}");
            return;
        }
    }
}
