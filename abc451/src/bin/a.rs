#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars
    }

    if S.len() % 5 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
