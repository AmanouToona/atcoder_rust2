#![allow(non_snake_case)]
use std::println;

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut ans = 0;
    for i in 1..N - 1 {
        if A[i - 1] < A[i] && A[i] > A[i + 1] {
            ans += 1;
        }
    }

    println!("{ans}");
}
