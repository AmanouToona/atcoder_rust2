#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    if S[N - 1] == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
