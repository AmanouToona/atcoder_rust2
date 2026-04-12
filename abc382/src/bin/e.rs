#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, X): (usize, usize),
        P: [f64; N],
    }

    let P: Vec<f64> = P.iter().map(|x| x / 100.).collect();
    // K[k] := パッケージを開けた際に、あたりを k 枚引く確率
    let mut K = vec![0.; N + 1];
    K[0] = 1.;
    for p in P.iter() {
        for j in (0..N).rev() {
            K[j + 1] += K[j] * p;
            K[j] *= 1. - p;
        }
    }

    // dp[x] := x枚のあたりを引いた時のパックの開封個数の期待値
    let mut dp = vec![0.; X + 1];
    for i in (0..X).rev() {
        let mut tmp = K[0];
        for (j, k) in K.iter().enumerate().skip(1) {
            if i + j <= X {
                tmp += (dp[i + j] + 1.) * k;
            } else {
                tmp += 1. * k;
            }
        }
        dp[i] = tmp / (1. - K[0]);
    }

    println!("{}", dp[0]);
}
