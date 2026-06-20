#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (X, Y): (usize, usize),
    }

    if X * 9 == Y * 16 {
        println!("Yes")
    } else {
        println!("No")
    }
}
