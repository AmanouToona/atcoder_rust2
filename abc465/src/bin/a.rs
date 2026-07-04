#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (A, B): (usize, usize),
    }

    if A * 3 > B * 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
