#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
fn main() {
    input! {
        (N, M): (usize, usize),
        uvw: [(usize, usize, i64); M],
    }

    let mut g = vec![Vec::new(); N];
    for &(u, v, w) in uvw.iter() {
        g[u - 1].push((v - 1, w));
    }

    let mut dp = vec![vec![1 << 60; 1 << N]; N];
    let mut q: BinaryHeap<(Reverse<i64>, usize, usize)> = BinaryHeap::new();
    for i in 0..N {
        dp[i][1 << i] = 0;
        q.push((Reverse(0), i, 1 << i));
    }

    while let Some((Reverse(cost), u, state)) = q.pop() {
        if dp[u][state] < cost {
            continue;
        }

        for &(v, w) in g[u].iter() {
            let nxt_state = state | (1 << v);
            let v_cost = cost + w;

            if dp[v][nxt_state] <= v_cost {
                continue;
            } else {
                dp[v][nxt_state] = v_cost;
                q.push((Reverse(v_cost), v, nxt_state));
            }
        }
    }

    let ans = dp.iter().map(|x| x[(1 << N) - 1]).min().unwrap();
    if ans == 1 << 60 {
        println!("No");
    } else {
        println!("{ans}");
    }
}
