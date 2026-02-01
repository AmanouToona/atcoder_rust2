#![allow(non_snake_case)]
use proconio::input;
fn dfs(i: usize, t: usize, dp: &mut Vec<Vec<usize>>, pab: &Vec<(usize, usize, usize)>) -> usize {
    if dp[t][i] != usize::MAX {
        return dp[t][i];
    }

    if pab[i].0 >= t {
        dp[t][i] = dfs(i + 1, t + pab[i].1, dp, pab);
        dp[t][i]
    } else {
        dp[t][i] = dfs(i + 1, t.saturating_sub(pab[i].2), dp, pab);
        dp[t][i]
    }
}

fn main() {
    input! {
        N: usize,
        PAB: [(usize, usize, usize); N],
        Q: usize,
    }

    let mut dp = vec![vec![usize::MAX; N + 1]; 1001];
    for i in 0..=1000 {
        dp[i][N] = i;
    }

    let mut minus = vec![0; N + 1];
    for (i, &(_, _, b)) in PAB.iter().enumerate() {
        minus[i + 1] += b;
        minus[i + 1] += minus[i];
    }

    for _ in 0..Q {
        input! {x: usize}

        if x >= minus[N] + 1000 {
            println!("{}", x - minus[N]);
            continue;
        }

        if x <= 1000 {
            let ans = dfs(0, x, &mut dp, &PAB);
            println!("{ans}");
            continue;
        }

        let mut left = 0;
        let mut right = N;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if minus[mid] > x - 1000 {
                right = mid;
            } else {
                left = mid;
            }
        }

        let ans = dfs(right, x - minus[right], &mut dp, &PAB);
        println!("{ans}");
    }
}
