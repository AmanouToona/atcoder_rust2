#![allow(non_snake_case)]
use proconio::input;
use std::collections::BTreeSet;
use std::ops::Bound::{Included, Unbounded};
fn main() {
    const TEN6: usize = 10usize.pow(6);
    let mut count = [0; TEN6 + 1];
    for i in 2..=TEN6 {
        if count[i] != 0 {
            continue;
        }

        count[i] += 1;
        let mut j = i + i;
        while j <= TEN6 {
            count[j] += 1;
            j += i;
        }
    }

    let set: BTreeSet<usize> = count
        .iter()
        .enumerate()
        .filter(|x| *x.1 == 2)
        .map(|x| x.0)
        .collect();

    input! {Q: usize}
    for _ in 0..Q {
        input! {A: usize};

        let ans = set
            .range((Unbounded, Included(A.isqrt())))
            .next_back()
            .unwrap();
        println!("{}", ans * ans);
    }
}
