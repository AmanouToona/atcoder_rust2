#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
dp[i][n] := i 日目に都市 n にいる時の組み合わせ
計算量は 5000 * 5000 * ... なので間に合わない

状態は
i 日目, 都市 n, 利用可能 or 不能な橋

ある都市に到達不能な都市 を数え上げれば高々 5000 個

*/
fn main() {
    input! {
        (N, M, K): (usize, usize ,usize),
        uv: [(usize, usize); M],
    }

    let mut cant = vec![Vec::new(); N];
    for &(u, v) in uv.iter() {
        cant[u - 1].push(v - 1);
        cant[v - 1].push(u - 1);
    }
    for i in 0..N {
        cant[i].push(i);
    }

    let mut dp = vec![mint::new(0); N];
    dp[0] = mint::new(1);
    for _ in 0..K {
        let mut nxt_dp = vec![mint::new(0); N];
        let sum: mint = dp.iter().sum();
        for (i, cnt_frm) in cant.iter().enumerate() {
            nxt_dp[i] += sum;
            for &f in cnt_frm {
                nxt_dp[i] -= dp[f];
            }
        }
        dp = nxt_dp;
    }

    println!("{}", dp[0]);
}
