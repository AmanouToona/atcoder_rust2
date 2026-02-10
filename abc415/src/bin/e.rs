#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (H, W): (usize, usize),
        A: [[i64; W]; H],
        P: [i64; H + W - 1],
    }

    let INF: i64 = 1_000_000_000_000_000;
    let mut dp = vec![vec![(-INF, -INF); W]; H];
    dp[0][0] = (A[0][0] - P[0], A[0][0] - P[0]);

    for h in 0..H {
        for w in 0..W {
            for &(dh, dw) in [(1, 0), (0, 1)].iter() {
                let vh = h + dh;
                let vw = w + dw;
                if vh >= H || vw >= W {
                    continue;
                }

                let day = vh + vw;
                let nxt_day_has = dp[h][w].0 + A[vh][vw] - P[day];
                let nxt_state = (nxt_day_has, dp[h][w].1.min(nxt_day_has));

                if nxt_state.1 > dp[vh][vw].1
                    || (nxt_state.1 == dp[vh][vw].1 && nxt_state.0 > dp[vh][vw].0)
                    || nxt_state.1 >= 0 && nxt_state.0 > dp[vh][vw].0
                {
                    dp[vh][vw] = nxt_state;
                }
            }
        }
    }

    let ans = -dp[H - 1][W - 1].1.min(0);
    println!("{ans}");
}
