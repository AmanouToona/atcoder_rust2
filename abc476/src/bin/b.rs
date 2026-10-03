#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        S: Chars,
        T: Chars,
    }

    for (&s, &t) in S.iter().zip(T.iter()) {
        if t == '*' {
            continue;
        }
        if s != t {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
