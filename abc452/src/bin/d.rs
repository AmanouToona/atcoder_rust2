#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let mut pair = Vec::new();
    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; T.len()]; S.len() + 1];

    for (i, &s) in S.iter().enumerate() {
        for j in 0..T.len() {
            dp[i + 1][j] = dp[i][j];
        }
        let pre = dp[i + 1][T.len() - 1];
        for (j, &t) in T.iter().enumerate().rev() {
            if j == 0 && s == t {
                dp[i + 1][0] = Some(i);
            }

            if j != 0 && t == s && dp[i][j - 1].is_some() {
                dp[i + 1][j] = dp[i][j - 1];
            }

            if j == T.len() - 1 && t == s && dp[i + 1][j].is_some() && dp[i + 1][j] != pre {
                pair.push((dp[i + 1][j].unwrap(), i));
            }
        }
    }

    let mut ans = (S.len() + 1) * S.len() / 2;
    for (i, &(l, r)) in pair.iter().enumerate() {
        let sub = if i == 0 {
            (l + 1) * (S.len() - r)
        } else {
            (l - pair[i - 1].0) * (S.len() - r)
        };
        ans -= sub;
    }
    println!("{ans}");
}
