#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M, L): (usize, usize, usize),
        A: [usize; N],
    }

    // L ごとのグループについて DP
    let mut dp = vec![vec![0; M]; N];
    for (i, a) in A.iter().enumerate().rev() {
        let mut j = i;
        loop {
            if j >= N - L {
                for m in 0..M {
                    dp[j][m] = (m + M - a) % M;
                }
            } else {
                for m in 0..M {
                    dp[j][m] = dp[j + L][m] + ((m + M) - a) % M;
                }
            }

            if j < L {
                break;
            }
            j -= L;
        }
    }

    // group cost
    let mut g_cost = Vec::new();
    for i in dp.iter().take(L) {
        g_cost.push(i.clone());
    }

    // for i in g_cost.iter() {
    //     println!("{:?}", i);
    // }

    let mut u_state = g_cost[0].clone();
    for vs in g_cost.iter().skip(1) {
        let mut v_state = vec![usize::MAX; M];
        for (u, u_cost) in u_state.iter().enumerate() {
            for (v, v_cost) in vs.iter().enumerate() {
                v_state[(v + u) % M] = v_state[(v + u) % M].min(*u_cost + v_cost);
            }
        }
        u_state = v_state;
        // eprintln!("{:?}", u_state);
    }

    let ans = u_state[0];
    println!("{ans}");
}
