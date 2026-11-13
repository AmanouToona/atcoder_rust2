#![allow(non_snake_case)]
use core::panic;
use std::{eprintln, println};

use num::Integer;
use proconio::input;

const UNVISIT: usize = usize::MAX;

/*
半開区間, [left, right)
(right - left) is even
return minimal cost
*/
fn small_cost(left: usize, right: usize, dp: &mut [Vec<usize>], a: &[usize]) -> usize {
    if left >= right {
        return 0;
    }

    if (right - left).is_odd() {
        panic!("{left} {right}");
    }

    if dp[left][right] != UNVISIT {
        return dp[left][right];
    }

    let mut best = usize::MAX;
    for i in (left + 1..right).step_by(2) {
        best = best.min(
            a[left].abs_diff(a[i])
                + small_cost(left + 1, i, dp, a)
                + small_cost(i + 1, right, dp, a),
        );
    }
    dp[left][right] = best;

    dp[left][right]
}

fn main() {
    input! {
        N: usize,
        A: [usize; 2 * N],
    }

    let mut dp = vec![vec![UNVISIT; 2 * N + 1]; 2 * N + 1];

    println!("{}", small_cost(0, 2 * N, &mut dp, &A));
}
