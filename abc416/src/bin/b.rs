#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut can = true;
    let mut T = Vec::new();
    for &s in S.iter() {
        if s == '#' {
            T.push('#');
            can = true;
            continue;
        }
        if can {
            T.push('o');
        } else {
            T.push('.');
        }
        can = false;
    }

    let ans: String = T.iter().join("");
    println!("{ans}");
}
