#![allow(non_snake_case)]
use proconio::input;

/*
レアが i 枚引いた時のパック開封数の期待値を
dp[i] とする
1パック開けた時のレアを期待値や確率は算出できる これは計算量 10**4
0..=100 の各確率を算出する

あとは期待値の dp 解ける
*/

fn main() {
    input! {
        (N, X): (usize, usize),
        P: [f64; N],
    }

    let mut rare_num: Vec<f64> = vec![0.; N + 1];
    rare_num[0] = 1.;
    for &p in P.iter() {
        let p = p / 100.;
        for i in (0..N).rev() {
            rare_num[i + 1] += rare_num[i] * p;
            rare_num[i] *= 1. - p;
        }
    }

    let mut dp = vec![0.; X + 1];
    for i in (0..X).rev() {
        let mut s = 0.;
        for j in 1..=N {
            if j + i <= X {
                s += (dp[i + j] + 1.) * rare_num[j];
            } else {
                s += rare_num[j];
            }
        }

        dp[i] = (s + rare_num[0]) / (1. - rare_num[0]);
    }
    println!("{}", dp[0]);
}
