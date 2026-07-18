#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        (N, M, Y): (usize, usize, usize),
        uvt: [(usize, usize, usize); M],
        x: [usize; N],
    }

    let mut g = vec![Vec::new(); N + 2];
    for &(u, v, t) in uvt.iter() {
        g[u - 1].push((v - 1, t));
        g[v - 1].push((u - 1, t));
    }

    for (i, &x) in x.iter().enumerate() {
        g[i].push((N, x));
        g[N + 1].push((i, x));
    }
    g[N].push((N + 1, Y));

    let mut distance = vec![usize::MAX; N + 2];
    distance[0] = 0;
    let mut q = BinaryHeap::new();
    for &(to, cost) in g[0].iter() {
        q.push((Reverse(cost), to));
    }

    while let Some((Reverse(cost), u)) = q.pop() {
        if distance[u] <= cost {
            continue;
        }

        distance[u] = cost;

        for &(v, d_cost) in g[u].iter() {
            let v_cost = cost + d_cost;
            if distance[v] <= v_cost {
                continue;
            }
            q.push((Reverse(v_cost), v));
        }
    }

    let ans = distance.iter().skip(1).take(N - 1).join(" ");
    println!("{ans}");

    // eprintln!("{:?}", distance);
}
