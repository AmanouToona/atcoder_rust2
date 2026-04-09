#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let mut ans = 0;
    let mut dp: Vec<Option<usize>> = vec![None; T.len()];
    for (i, &s) in S.iter().enumerate() {
        for (j, &t) in T.iter().enumerate().skip(1).rev() {
            if t == s && dp[j - 1].is_some() {
                dp[j] = dp[j - 1];
            }
        }
        if s == T[0] {
            dp[0] = Some(i);
        }

        let invalid_count = match dp.last().unwrap() {
            Some(l) => l + 1,
            None => 0,
        };

        ans += i + 1 - invalid_count;
    }

    println!("{ans}");
}
