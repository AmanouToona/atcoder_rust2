#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;
fn main() {
    input! {
        Q: usize,
        S: Chars,
        T: Chars,
    }

    let mut start_pos = BTreeSet::new();
    if S.len() >= T.len() {
        'outer: for i in 0..=S.len() - T.len() {
            for j in 0..T.len() {
                if S[i + j] != T[j] {
                    continue 'outer;
                }
            }
            start_pos.insert(i);
        }
    }

    for _ in 0..Q {
        input! {(L, R): (usize, usize)}
        let L = L - 1;
        let R = R - 1;
        match start_pos.range(L..=R).next() {
            Some(&start) => {
                if start + T.len() - 1 <= R {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {
                println!("No")
            }
        };
    }
}
