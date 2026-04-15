#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    for (i, (&s, &t)) in S.iter().zip(T.iter()).enumerate() {
        if s != t {
            println!("{}", i + 1);
            return;
        }
    }

    println!("{}", T.len());
}
