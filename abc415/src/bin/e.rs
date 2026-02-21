#![allow(non_snake_case)]
use proconio::input;

fn can_reach(x: i64, A: &[Vec<i64>], P: &[i64]) -> bool {
    let H = A.len();
    let W = A[0].len();
    let INF: i64 = 1_000_000_000_000_000;
    let mut dp = vec![vec![-INF; W]; H];
    dp[0][0] = A[0][0] + x - P[0];

    for h in 0..H {
        for w in 0..W {
            for &(dh, dw) in [(1, 0), (0, 1)].iter() {
                let vh = h + dh;
                let vw = w + dw;
                if vh >= H || vw >= W {
                    continue;
                }
                if dp[h][w] < 0 {
                    continue;
                }

                let day = vh + vw;
                dp[vh][vw] = dp[vh][vw].max(dp[h][w] + A[vh][vw] - P[day]);
            }
        }
    }

    dp[H - 1][W - 1] >= 0
}

fn main() {
    input! {
        (H, W): (usize, usize),
        A: [[i64; W]; H],
        P: [i64; H + W - 1],
    }

    if can_reach(0, &A, &P) {
        println!("0");
        return;
    }

    let INF: i64 = 1_000_000_000_000_000;
    let mut left = 0;
    let mut right = INF;

    can_reach(20, &A, &P);

    while right - left > 1 {
        let mid = (right + left) / 2;
        if can_reach(mid, &A, &P) {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{right}");
}
