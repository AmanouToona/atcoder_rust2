#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    if S[0].is_lowercase() {
        println!("No");
        return;
    }

    for &s in S.iter().skip(1) {
        if s.is_uppercase() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
