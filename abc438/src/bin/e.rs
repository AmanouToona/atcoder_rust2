use std::usize;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        A: [usize; N],
        TB: [(usize, usize); Q],
    }

    // ダブリング
    let mut dp = vec![vec![0; N]; 30]; // 10 ** 9 なので、 30　で十分
    for (i, a) in A.iter().enumerate() {
        dp[0][i] = a - 1;
    }
    for i in 1..30 {
        for j in 0..N {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    // 水の量のダブリング
    let mut bucket = vec![vec![0; N]; 30];
    for i in 0..N {
        bucket[0][i] = i + 1;
    }
    for i in 1..30 {
        for j in 0..N {
            bucket[i][j] = bucket[i - 1][j] + bucket[i - 1][dp[i - 1][j]];
        }
    }

    for &(t, b) in TB.iter() {
        let mut ans = 0;
        let mut pos = b - 1;
        for i in 0..30 {
            if t >> i & 1 == 0 {
                continue;
            }
            ans += bucket[i][pos];
            pos = dp[i][pos];
        }
        println!("{ans}");
    }
}
