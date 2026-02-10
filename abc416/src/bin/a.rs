#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (_, L, R): (usize, usize, usize),
        S: Chars,
    }

    for &s in S.iter().skip(L - 1).take(R - L + 1) {
        if s == 'x' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
