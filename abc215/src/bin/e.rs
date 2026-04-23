#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use az::UnwrappedAs;
use proconio::input;
use proconio::marker::Chars;
/*
dp だとすると、最後に出たコンテスト, すでに出たコンテストの情報は必要.
すでに出たコンテストに出場できるのは最後に出たコンテストと同一である場合のみ.
dp[i個目まで見た 1000][すでに出たコンテスト ... 1<<10][最後に出たコンテスト 10] := 選び方
これで計算量は 1000 * (1000 * 10) * 10 ... ちょっと危ない
*/
fn main() {
    input! {
        _: usize,
        S: Chars,
    }

    let S: Vec<usize> = S
        .iter()
        .map(|s| (*s).to_lowercase().next().unwrap())
        .map(|s| (s as u8 - b'a') as usize)
        .collect();

    let mut dp = vec![vec![mint::new(0); 10]; 1 << 10];
    dp[0][0] = mint::new(1);
    for &s in S.iter() {
        let mut nxt = dp.clone();
        for state in 0..1 << 10 {
            for pre_c in 0..10 {
                if state >> s & 1 == 1 && pre_c != s {
                    continue;
                }
                let nxt_state = state | 1 << s;
                nxt[nxt_state][s] += dp[state][pre_c];
            }
        }
        dp = nxt;
    }

    let mut ans = mint::new(0);
    for state in 1..1 << 10 {
        for c in 0..10 {
            ans += dp[state][c];
        }
    }

    println!("{ans}");
}
