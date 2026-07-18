#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
use proconio::marker::Chars;

/*
普通にDPなら持つ状態の数は
- 利用した数字 1000
- あまり 3
- 桁数 500
- 以上以下 2
計 : 6 * 10 ** 6

ギリ間に合う

*/

fn count_one(n: usize) -> usize {
    let mut res = 0;
    let mut n = n;
    while n != 0 {
        n = n & (n - 1);

        res += 1
    }

    res
}

fn main() {
    input! {
        N: Chars,
    }

    // 桁 DP
    // dp[使った数 bit][あまり][Nより小さい 1: なら小さい][数字が立った]
    let mut dp = vec![vec![vec![vec![mint::new(0); 2]; 2]; 3]; 1 << 10];
    dp[0][0][0][0] = mint::new(1);

    for n in N.iter() {
        let mut nxt_dp = vec![vec![vec![vec![mint::new(0); 2]; 2]; 3]; 1 << 10];

        let n: usize = n.to_digit(10).unwrap() as usize;
        for i in 0..10usize {
            for state in 0..1 << 10 {
                for res in 0..3 {
                    for smaller in 0..2 {
                        if smaller == 0 && i > n {
                            continue;
                        }
                        for not_zero in 0..2 {
                            if dp[state][res][smaller][not_zero] == mint::new(0) {
                                continue;
                            }
                            let nxt_zero = if not_zero == 1 || i != 0 { 1 } else { 0 };

                            let nxt_smaller = if smaller == 1 || i < n { 1 } else { 0 };

                            let nxt_state = if not_zero == 0 && i == 0 {
                                0
                            } else {
                                state | (1 << i)
                            };

                            nxt_dp[nxt_state][(res * 10 + i) % 3][nxt_smaller][nxt_zero] +=
                                dp[state][res][smaller][not_zero];
                        }
                    }
                }
            }
        }
        dp = nxt_dp;
    }

    let mut ans = mint::new(0);

    for state in 1..1 << 10 {
        // 3 の倍数
        if count_one(state) != 3 && (state & (1 << 3) == 0) {
            ans += dp[state][0][0][1];
            ans += dp[state][0][1][1];
        }

        // 3 が含まれる
        if count_one(state) != 3 && (state & (1 << 3) != 0) {
            for res in 1..=2 {
                ans += dp[state][res][0][1];
                ans += dp[state][res][1][1];
            }
        }

        // 3 種類の数字
        if count_one(state) == 3 && (state & (1 << 3) == 0) {
            for res in 1..=2 {
                ans += dp[state][res][0][1];
                ans += dp[state][res][1][1];
            }
        }
    }

    println!("{}", ans);
}
