#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
fn main() {
    input! {
        (N, M, K): (usize, usize ,usize),
        uv: [(usize, usize); M],
    }

    let mut dp = vec![vec![mint::new(0); N]; K + 1];
    dp[0][0] = mint::new(1);

    for i in 1..=K {
        let sum = dp[i - 1].iter().sum::<mint>();
        for j in 0..N {
            dp[i][j] = sum - dp[i - 1][j];
        }

        for &(u, v) in uv.iter() {
            let u = u - 1;
            let v = v - 1;

            let pre_u = dp[i - 1][u];
            let pre_v = dp[i - 1][v];
            dp[i][v] -= pre_u;
            dp[i][u] -= pre_v;
        }
    }

    println!("{}", dp[K][0]);
}
