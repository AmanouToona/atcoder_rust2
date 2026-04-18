#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
確率遷移の問題
N は十分に小さいので DP 的な確率遷移で解ける
dp[モンスターの体力] = 期待値として
dp[0], dp[1] だけは決定できている状態から開始 dp[N]が答え
*/
fn main() {
    input! {
        (N, P): (usize, usize)
    }

    let mut dp = vec![mint::new(0); N + 1];
    let p = mint::new(P) / mint::new(100);
    dp[1] = mint::new(1);

    for i in 2..=N {
        dp[i] = (mint::new(1) - p) * (dp[i - 1] + 1) + p * (dp[i - 2] + 1);
    }

    println!("{}", dp[N]);
}
