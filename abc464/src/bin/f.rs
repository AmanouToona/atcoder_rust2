#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
use std::collections::HashMap;
/*
半分全探索?

*/
fn main() {
    input! {
        (N, X): (usize, usize),
        A: [usize; N],
    }

    let (h1, h2) = A.split_at(N / 2);

    let mut cnt1: HashMap<usize, usize> = HashMap::new();
    'outer: for i in 0..1 << h1.len() {
        let mut sum = 0;
        for bit in 0..h1.len() {
            if sum >= X {
                continue 'outer;
            }
            if i >> bit & 1 == 1 {
                sum += h1[bit];
            }
        }
        *cnt1.entry(sum).or_default() += 1;
    }

    let mut cnt2: HashMap<usize, usize> = HashMap::new();
    'outer: for i in 0..1 << h2.len() {
        let mut sum = 0;
        for bit in 0..h2.len() {
            if sum >= X {
                continue 'outer;
            }
            if i >> bit & 1 == 1 {
                sum += h2[bit];
            }
        }
        *cnt2.entry(sum).or_default() += 1;
    }

    let mut c2v: Vec<usize> = cnt2.keys().cloned().collect();
    c2v.sort();
    let mut idx_combination = vec![0; cnt2.len() + 1];
    for (i, &v) in c2v.iter().enumerate() {
        idx_combination[i + 1] = idx_combination[i - 1] + cnt2[&v];
    }

    let e = mint::new(0);
    for (k1, v1) in cnt1.iter() {
        let idx = c2v.partition_point(|&x| x + k1 < X);
        if idx >= c2v.len() {
            continue;
        }
    }
}
