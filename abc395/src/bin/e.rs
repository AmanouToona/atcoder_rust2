#![allow(non_snake_case)]
use std::cmp::Reverse;

use proconio::input;
fn main() {
    input! {
        (N, M, X): (usize, usize, usize),
        uv: [(usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];
    let mut g_rev = vec![Vec::new(); N];

    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push(v);
        g_rev[v].push(u);
    }

    let mut q: std::collections::BinaryHeap<(Reverse<usize>, usize, bool)> =
        std::collections::BinaryHeap::new();
    q.push((Reverse(0), 0, false));
    let max = 1 << 60;
    let mut node = vec![vec![max; 2]; N];

    while let Some((Reverse(cost), u, rev)) = q.pop() {
        if node[u][rev as usize] < cost {
            continue;
        }

        node[u][rev as usize] = cost;

        let (forward, reverse) = if !rev {
            (&g[u], &g_rev[u])
        } else {
            (&g_rev[u], &g[u])
        };

        for &v in forward {
            let nxt_cost = cost + 1;
            if node[v][rev as usize] > nxt_cost {
                q.push((Reverse(nxt_cost), v, rev));
                node[v][rev as usize] = nxt_cost;
            }
        }

        for &v in reverse {
            let nxt_cost = cost + X + 1;
            if node[v][!rev as usize] > nxt_cost {
                q.push((Reverse(nxt_cost), v, !rev));
                node[v][!rev as usize] = nxt_cost;
            }
        }
    }

    println!("{}", node[N - 1].iter().min().unwrap());
}
