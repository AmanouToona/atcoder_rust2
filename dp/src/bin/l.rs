#![allow(non_snake_case)]
use proconio::input;

fn max_score(
    left: usize,
    right: usize,
    dp: &mut [Vec<Option<i64>>],
    prefix_sum: &[i64],
    a: &[i64],
) -> i64 {
    if left == right {
        return 0;
    }
    if let Some(score) = dp[left][right] {
        return score;
    }

    dp[left][right] = Some(
        (-max_score(left + 1, right, dp, prefix_sum, a)).max(-max_score(
            left,
            right - 1,
            dp,
            prefix_sum,
            a,
        )) + prefix_sum[right]
            - prefix_sum[left],
    );

    dp[left][right].unwrap()
}

fn main() {
    input! {
        N: usize,
        A: [i64; N],
    }

    let mut prefix_sum = vec![0];
    for &a in A.iter() {
        prefix_sum.push(a + prefix_sum.last().unwrap());
    }

    let mut dp: Vec<Vec<Option<i64>>> = vec![vec![None; N + 1]; N + 1];

    let x = max_score(0, N, &mut dp, &prefix_sum, &A);
    let y = prefix_sum[N] - x;
    println!("{}", x - y);
}
