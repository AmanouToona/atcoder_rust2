#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

/*
s が 10 ** 3 なら DP で解ける
10 ** 4 でもギリギリ解けるか?

dp[i][j] := i, j を回文にするのに必要な操作回数
dp[i][i] = 0 で初期化
dp[i][i + 1] も初期化しておく

dp[i][j] = dp[i + 1][j - 1]  ... if (S[i] == S[j])
dp[i][j] = min(dp[i + 1][j], dp[i][j - 1]) + 1 ... if (S[i] != S[j])

*/
fn main() {
    input! {
        S: Chars,
    }

    let n = S.len();
    let mut dp = vec![vec![usize::MAX; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }

    for i in 0..n - 1 {
        if S[i] == S[i + 1] {
            dp[i][i + 1] = 0;
        } else {
            dp[i][i + 1] = 1;
        }
    }

    for len in 2..n {
        for i in 0..n - len {
            let j = i + len;
            if S[i] == S[j] {
                dp[i][j] = dp[i + 1][j - 1];
            } else {
                dp[i][j] = dp[i + 1][j - 1] + 1;
            }
        }
    }

    // for i in dp.iter() {
    //     println!("{:?}", i);
    // }

    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            if dp[i][j] <= 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
