#![allow(non_snake_case)]
use proconio::input;

const UNVISIT: usize = usize::MAX;

fn small_cost(left: usize, right: usize, dp: &mut [Vec<usize>], prefix_sum: &[usize]) -> usize {
    if right - left <= 1 {
        return 0;
    }

    if dp[left][right] != UNVISIT {
        return dp[left][right];
    }

    let mut best = usize::MAX;
    for i in left + 1..right {
        best = best.min(small_cost(left, i, dp, prefix_sum) + small_cost(i, right, dp, prefix_sum));
    }
    dp[left][right] = best + prefix_sum[right] - prefix_sum[left];

    dp[left][right]
}

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut prefix_sum = vec![0];
    for &a in A.iter() {
        prefix_sum.push(prefix_sum.last().unwrap() + a);
    }

    // [l, r) combine smallest cost.
    let mut dp = vec![vec![UNVISIT; N + 1]; N + 1];

    println!("{}", small_cost(0, N, &mut dp, &prefix_sum));
}
