#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut cnt = HashMap::new();
    for i in [1, 10, 100] {
        cnt.insert(i, 0);
    }

    for &a in A.iter() {
        let mut res = 1000 * a.div_ceil(1000) - a;
        for coin in [100, 10, 1] {
            *cnt.entry(coin).or_default() += res / coin;
            res -= res / coin * coin;
        }
    }

    println!("{} {} {}", cnt[&1], cnt[&10], cnt[&100]);
}
