use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        uv: [(usize, usize); M],
        S: Chars,
    }

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); N];

    for &(u, v) in uv.iter() {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let mut q = VecDeque::new();
    let mut dist: Vec<Vec<usize>> = vec![Vec::new(); N];
    for (i, s) in S.iter().enumerate() {
        if *s == 'S' {
            q.push_back((i, 0));
            dist[i].push(0);
        }
    }

    while let Some((u, d)) = q.pop_front() {
        for v in g[u].iter() {
            if dist[*v].len() >= 2 {
                continue;
            }
            dist[*v].push(d + 1);
            q.push_back((*v, d + 1));
        }
    }

    for (i, &s) in S.iter().enumerate() {
        if s == 'D' {
            let d: usize = dist[i].iter().sum();
            println!("{d}");
        }
    }
}
