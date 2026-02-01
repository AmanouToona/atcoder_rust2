#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        N: usize,
        A: [i64; N],
    }

    // Ai + i = j - Aj;
    let mut left: HashMap<i64, usize> = HashMap::new();
    for (i, &a) in A.iter().enumerate() {
        *left.entry(i as i64 + 1 + a).or_default() += 1;
    }

    let mut right: HashMap<i64, usize> = HashMap::new();
    for (j, &a) in A.iter().enumerate() {
        *right.entry(j as i64 + 1 - a).or_default() += 1;
    }

    let mut ans = 0;
    for (&k, &v) in left.iter() {
        ans += right.get(&k).unwrap_or(&0) * v;
    }

    println!("{ans}");
}
