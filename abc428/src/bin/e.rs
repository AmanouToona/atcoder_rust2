use proconio::input;
use std::{collections::VecDeque, mem};

fn bfs(u: usize, g: &[Vec<usize>]) -> Vec<usize> {
    let mut dist = vec![usize::MAX; g.len()];
    let mut q = VecDeque::new();
    dist[u] = 0;
    q.push_back(u);

    while let Some(u) = q.pop_front() {
        let nxt_dixt = dist[u] + 1;
        for &v in g[u].iter() {
            if dist[v] <= nxt_dixt {
                continue;
            }
            q.push_back(v);
            dist[v] = nxt_dixt
        }
    }
    dist
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N - 1],
    }

    let mut g = vec![Vec::new(); N];
    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;
        g[a].push(b);
        g[b].push(a);
    }

    // 直径の端点探索
    let dist = bfs(0, &g);

    let mut u = dist
        .iter()
        .enumerate()
        .max_by_key(|&(i, &d)| (d, i))
        .unwrap()
        .0;

    let mut dist_u = bfs(u, &g);

    let mut v = dist_u
        .iter()
        .enumerate()
        .max_by_key(|&(i, &d)| (d, i))
        .unwrap()
        .0;

    let mut dist_v = bfs(v, &g);
    // v の方が大きい
    if v < u {
        mem::swap(&mut dist_v, &mut dist_u);
        mem::swap(&mut v, &mut u);
    }

    for i in 0..N {
        if dist_v[i] >= dist_u[i] {
            println!("{}", v + 1);
        } else {
            println!("{}", u + 1);
        }
    }
}
