#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let mut dp = vec![vec![usize::MAX; N + 1]; N + 1];
    dp[0] = vec![0; N + 1];

    for j in dp.iter_mut() {
        j[0] = 0;
    }

    for i in 1..=N {
        // j列目までを白色にする
        // 見ている列以降の全てを黒にした時のコスト
        let mut blacken_cost: usize = S[i - 1]
            .iter()
            .fold(0usize, |sum, &x| sum + if x == '.' { 1 } else { 0 });

        let mut pre_min = dp[i - 1].clone();
        for i in (0..N).rev() {
            pre_min[i] = pre_min[i].min(pre_min[i + 1]);
        }

        dp[i][0] = pre_min[0] + blacken_cost;

        for (j, pre) in pre_min.iter().enumerate().skip(1) {
            if S[i - 1][j - 1] == '#' {
                blacken_cost += 1
            } else {
                blacken_cost -= 1
            }

            dp[i][j] = blacken_cost + pre;
        }
    }
    let ans = dp[N].iter().min().unwrap();
    println!("{ans}");
}
