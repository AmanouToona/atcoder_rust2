#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        P: Chars,
        L: usize,
    }

    if P.len() >= L {
        println!("Yes");
    } else {
        println!("No");
    }
}
