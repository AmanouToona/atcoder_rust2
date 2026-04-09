#![allow(non_snake_case)]
use num::Integer;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut offset = 0;
    for (i, s) in S.iter().enumerate() {
        if *s == 'i' {
            if (i + offset).is_odd() {
                offset += 1;
            }
        } else {
            if (i + offset).is_even() {
                offset += 1;
            }
        }
    }

    if (S.len() + offset).is_odd() {
        offset += 1;
    }

    println!("{offset}");
}
