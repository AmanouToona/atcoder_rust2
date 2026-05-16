#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
use proconio::marker::Chars;
/*
最後の文字が i である時の組み合わせを
dp[i] で持って更新
*/

fn main() {
    input! {
        S: Chars,
    }

    let mut dp = vec![mint::new(0); 3];

    for &s in S.iter() {
        let mut nxt_dp = dp.clone();

        let to = match s {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => panic!(),
        };

        for frm in 0..3 {
            if frm != to {
                nxt_dp[to] += dp[frm];
            }
        }
        nxt_dp[to] += mint::new(1);
        dp = nxt_dp;
    }

    let ans = dp.iter().sum::<mint>();
    println!("{ans}");
}
