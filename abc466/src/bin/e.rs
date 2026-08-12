#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, K) : (usize, usize),
        AB: [(i64, i64); N]
    }

    // dp[i][j][k] := i枚目まで見て、j(0: 表区間) k回反転 状態での最大和
    let mut dp = vec![vec![vec![i64::MIN; K + 1]; 2]; N + 1];
    dp[0][0][0] = 0;
    for (i, &(a, b)) in AB.iter().enumerate() {
        for k in 0..=K {
            // select a
            dp[i + 1][0][k] = dp[i + 1][0][k].max(dp[i][0][k] + a).max(dp[i][1][k] + a);

            // select b
            dp[i + 1][1][k] = dp[i + 1][1][k].max(dp[i][1][k] + b);
            if k < K {
                dp[i + 1][1][k + 1] = dp[i + 1][1][k + 1].max(dp[i][0][k] + b);
            }
        }
    }
    let ans = dp[N][0]
        .iter()
        .max()
        .unwrap()
        .max(dp[N][1].iter().max().unwrap());
    println!("{ans}");
}
