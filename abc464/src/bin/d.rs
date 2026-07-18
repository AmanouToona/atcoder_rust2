#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

/*
dp[i日目][i-1日目が雨] := i日目の嬉しさの最大値

dp[i][true] = dp[i][]

*/

fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            N: usize,
            S: Chars,
            X: [i128; N],
            Y: [i128; N - 1],
        }

        let mut dp: Vec<Vec<i128>> = vec![vec![-i128::MAX; 2]; N + 1];
        if S[0] == 'S' {
            dp[0][1] = 0;
            dp[0][0] = -X[0];
        } else {
            dp[0][0] = 0;
            dp[0][1] = -X[0];
        }

        for day in 1..N {
            if S[day] == 'S' {
                dp[day][1] = dp[day - 1][1].max(dp[day - 1][0] + Y[day - 1]);
                dp[day][0] = dp[day][0].max(dp[day - 1][1]).max(dp[day - 1][0]) - X[day];
            } else {
                dp[day][1] = dp[day - 1][1].max(dp[day - 1][0] + Y[day - 1]) - X[day];
                dp[day][0] = dp[day][0].max(dp[day - 1][1]).max(dp[day - 1][0]);
            }
        }

        let ans: &i128 = dp[N - 1].iter().max().unwrap();
        println!("{ans}");
    }
}
