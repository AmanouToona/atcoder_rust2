#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input!{
        A: usize
    }

    if 400 % A == 0 {
        println!("{}",  400/ A);
    } else {
        println!("-1");
    }
}
