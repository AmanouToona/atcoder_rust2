#![allow(non_snake_case)]

use proconio::input;
fn main() {
    /*
    最大値の最小化 ... 二分探索？
    dp でもできそう [iまで見た][x個食べた][y個食べた] = 食べた弁当の数 ... xy の最大は300でいいから状態は 300 * 300 * 300
    */
    input! {
        N: usize,
        (X, Y): (usize, usize),
        ab: [(usize, usize); N],
    }

    let mut dp = vec![vec![vec![usize::MAX; Y + 1]; X + 1]; N + 1];
    dp[0][0][0] = 0;
    for n in 0..N {
        for x in 0..=X {
            for y in 0..=Y {
                if dp[n][x][y] == usize::MAX {
                    continue;
                }
                // 食べない
                dp[n + 1][x][y] = dp[n + 1][x][y].min(dp[n][x][y]);

                // 食べる
                let nxt_x = X.min(x + ab[n].0);
                let nxt_y = Y.min(y + ab[n].1);
                dp[n + 1][nxt_x][nxt_y] = dp[n + 1][nxt_x][nxt_y].min(dp[n][x][y] + 1);
            }
        }
    }

    if dp[N][X][Y] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", dp[N][X][Y]);
    }
}
