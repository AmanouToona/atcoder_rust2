#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main() {
    input! {
        S: Chars,
    }

    let mut cnt: HashMap<char, usize> = HashMap::new();

    for &s in S.iter() {
        *cnt.entry(s).or_default() += 1;
    }

    for (k, v) in cnt.iter() {
        if *v == 1 {
            println!("{k}");
        }
    }
}
