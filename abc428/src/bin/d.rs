#![allow(non_snake_case)]
use proconio::input;

fn isqlt(n: usize) -> usize {
    let mut left = 0;
    let mut right = 10usize.pow(10);

    while right - left > 1 {
        let mid = (right + left) / 2;

        if mid * mid <= n {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

fn count(c: usize, limit: usize) -> usize {
    let mut res = 0;

    let mut k = 10; // y * k + x ... k は桁数を決める
    while k / 10 <= limit {
        let L = c * k + k / 10;
        let R = c * k + (k - 1).min(limit);

        res += isqlt(R) - isqlt(L - 1);
        k *= 10;
    }

    res
}

fn solve() {
    input! {
        (C, D): (usize, usize)
    }

    let ans = count(C, C + D) - count(C, C - 1);
    println!("{ans}");
}

fn main() {
    input! {
        T: usize
    }
    for _ in 0..T {
        solve();
    }
}
