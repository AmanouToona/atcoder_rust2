#![allow(non_snake_case)]
use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }

    let mut left: BTreeMap<usize, usize> = BTreeMap::new();
    let mut mid: BTreeMap<usize, usize> = BTreeMap::new();
    let mut right: BTreeMap<usize, usize> = BTreeMap::new();

    for (i, &a) in A.iter().enumerate() {
        if i < K {
            *mid.entry(a).or_default() += 1;
        } else {
            *right.entry(a).or_default() += 1;
        }
    }

    if mid.keys().last().unwrap() <= right.keys().next().unwrap() {
        println!("Yes");
        return;
    }

    for l in 0..N - K {
        let r = l + K;

        *left.entry(A[l]).or_default() += 1;

        *right.entry(A[r]).or_default() -= 1;
        if right[&A[r]] == 0 {
            right.remove(&A[r]);
        }

        *mid.entry(A[l]).or_default() -= 1;
        if mid[&A[l]] == 0 {
            mid.remove(&A[l]);
        }
        *mid.entry(A[r]).or_default() += 1;

        let mid_min = mid.keys().next().unwrap();
        let mid_max = mid.keys().last().unwrap();
        let right_min = right.keys().next().unwrap_or(&usize::MAX);
        let left_max = left.keys().last().unwrap();

        if left_max <= mid_min && mid_max <= right_min {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
