#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, X): (usize, usize),
        SCP: [(f64, usize, f64 ); N],
    }

    // dp[bit][money] = E :bit=解いた問題の状態, money=残金, E=期待値
    let mut dp: Vec<Vec<f64>> = vec![vec![0.; X + 1]; 1 << N];

    for bit in (0..1 << N).rev() {
        for money in 0..=X {
            for (i, &(s, c, p)) in SCP.iter().enumerate() {
                // block
                // すでに解いてある
                if (bit >> i) & 1 == 1 {
                    continue;
                }

                // 残金が足りない
                if money < c {
                    continue;
                }

                // 更新
                dp[bit][money] = dp[bit][money].max(
                    (dp[bit + (1 << i)][money - c] + s) * (p / 100.)
                        + dp[bit][money - c] * (1. - p / 100.),
                );
            }
        }
    }
    println!("{}", dp[0][X]);
}
