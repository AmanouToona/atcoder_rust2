#![allow(non_snake_case)]
use proconio::input;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn expgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, s, t) = expgcd(b, a % b);
        (g, t, s - a / b * t)
    }
}

fn solve(N: i64, S: i64, K: i64) {
    let g = gcd(K, N);
    if S % g != 0 {
        println!("-1");
        return;
    }

    let N = N / g;
    let S = S / g;
    let K = K / g;

    let (a, x, y) = expgcd(K, N);
    eprintln!("{x} {y} {S} {g}");
    let ans = (-x * S % N + N) % N;
    println!("{ans}");
}

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {(N, S, K): (i64, i64, i64)}
        solve(N, S, K);
    }
}
