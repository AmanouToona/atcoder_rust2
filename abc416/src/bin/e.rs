#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let INF = 1_000_000_000_000;

    // 空港を N 番目の街として処理する
    // 空港までの経路にTを押し付ける
    let mut dp = vec![vec![INF; N + 1]; N + 1];
    for i in 0..=N {
        dp[i][i] = 0;
    }

    for _ in 0..M {
        input! {
            (A, B, C): (usize, usize, usize),
        }
        let A = A - 1;
        let B = B - 1;
        dp[A][B] = dp[A][B].min(C * 2);
        dp[B][A] = dp[B][A].min(C * 2);
    }

    input! {
        (K, T): (usize, usize),
        D: [usize; K]
    }

    for &d in D.iter() {
        let d = d - 1;
        dp[d][N] = T;
        dp[N][d] = T;
    }

    // ワーシャルフロイド
    for k in 0..=N {
        for i in 0..=N {
            for j in 0..=N {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }

    input! {Q: usize}
    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                input! {(x, y, t): (usize, usize, usize)}
                let x = x - 1;
                let y = y - 1;
                dp[x][y] = dp[x][y].min(t * 2);
                dp[y][x] = dp[y][x].min(t * 2);

                for i in 0..=N {
                    for j in 0..=N {
                        dp[i][j] = dp[i][j]
                            .min(dp[i][x] + dp[x][y] + dp[y][j])
                            .min(dp[i][y] + dp[y][x] + dp[x][j]);
                        dp[j][i] = dp[j][i].min(dp[i][j]);
                    }
                }
            }
            2 => {
                input! {x: usize}
                let x = x - 1;
                dp[x][N] = T;
                dp[N][x] = T;

                for i in 0..=N {
                    for j in 0..=N {
                        dp[i][j] = dp[i][j]
                            .min(dp[i][x] + dp[x][N] + dp[N][j])
                            .min(dp[i][N] + dp[N][x] + dp[x][j]);
                        dp[j][i] = dp[j][i].min(dp[i][j]);
                    }
                }
            }
            3 => {
                let mut ans = 0;
                for i in 0..N {
                    for j in 0..N {
                        if dp[i][j] != INF {
                            ans += dp[i][j];
                        }
                    }
                }
                println!("{}", ans / 2);
            }
            _ => {
                panic!("wrong query!!")
            }
        }
    }
}
