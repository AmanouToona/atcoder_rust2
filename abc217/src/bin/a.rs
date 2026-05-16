#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    if S < T {
        println!("Yes");
    } else {
        println!("No");
    }
}
