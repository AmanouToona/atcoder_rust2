#![allow(non_snake_case)]
use std::usize;

use proconio::input;

fn main() {
    input! {
        (N, K): (usize, usize),
        mut A: [usize; N],
    }

    A.sort();
    let amax = A.last().unwrap();

    let mut max = amax + K + (K + 1) / 2;
    let mut min = max - K;

    let mut ans = usize::MAX;
    while max - min > 1 {
        let mid = (max + min) / 2;

        let mut tmp = Vec::new();
        for &a in A.iter() {
            let div = (mid - a) / K;
            tmp.push((a + div * K));
        }

        let tmp = tmp.iter().max().unwrap() - tmp.iter().min().unwrap();

        if tmp < ans {
            max = mid;
            ans = tmp;
        } else {
            min = mid;
        }
    }
    println!("{ans}");
}
