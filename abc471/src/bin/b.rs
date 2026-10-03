#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }
    let mut count: HashMap<String, usize> = HashMap::new();
    for s in S.iter() {
        *count
            .entry(s.iter().cloned().map(|c| c.to_ascii_lowercase()).join(""))
            .or_default() += 1;
    }

    let ans = count.values().max().unwrap_or(&1);
    println!("{ans}");
}
