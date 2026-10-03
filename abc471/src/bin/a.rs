#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (A, B): (usize, usize)
    }

    if A + B == 9 || A == 9 + B || A * B == 9 || A == B * 9 {
        println!("Nine");
    } else {
        println!("Nein");
    }
}
