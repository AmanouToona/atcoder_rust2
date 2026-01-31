#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        _: usize,
        S: Chars,
    }

    if S.ends_with(&"tea".chars().collect::<Vec<char>>()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
