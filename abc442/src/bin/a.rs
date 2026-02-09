#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }
    let mut ans = 0;
    for &s in S.iter() {
        if s == 'i' || s == 'j' {
            ans += 1;
        }
    }

    println!("{ans}")
}
