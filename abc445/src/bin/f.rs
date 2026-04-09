use std::usize;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        C: [[usize; N]; N],
    }

    //　dubling
    // dp[k][from][to] = min cost from to to with 2 ** k node
    let mut dp = vec![vec![vec![usize::MAX; N]; N]; 31];
    for i in 0..N {
        for j in 0..N {
            dp[0][i][j] = C[i][j];
        }
    }

    // dubling
    for k in 0..29 {
        for frm in 0..N {
            for to in 0..N {
                for mid in 0..N {
                    dp[k + 1][frm][to] = dp[k + 1][frm][to].min(dp[k][frm][mid] + dp[k][mid][to]);
                }
            }
        }
    }

    // with K move
    let mut pre = vec![vec![0; N]; N];
    for bit in 0..=30 {
        let mut new = vec![vec![1 << 60; N]; N];
        if K >> bit & 1 == 0 {
            continue;
        }

        if 1 << bit == (!K + 1) & K {
            pre = dp[bit].clone();
            continue;
        }

        for frm in 0..N {
            for to in 0..N {
                for mid in 0..N {
                    new[frm][to] = new[frm][to].min(pre[frm][mid] + dp[bit][mid][to]);
                }
            }
        }
        pre = new;
    }

    for i in 0..N {
        println!("{}", pre[i][i]);
    }
}
