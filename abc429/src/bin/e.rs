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
    let mut dist: Vec<Vec<(usize, usize)>> = vec![Vec::new(); N];
    for (i, s) in S.iter().enumerate() {
        if *s == 'S' {
            q.push_back((i, 0, i)); // u, dist, root
            dist[i].push((0, i)); // dist, root
        }
    }

    while let Some((u, d, r)) = q.pop_front() {
        for v in g[u].iter() {
            if dist[*v].len() >= 2 {
                continue;
            }
            if let Some(&(_, i)) = dist[*v].first() {
                if i == r {
                    continue;
                }
            }

            dist[*v].push((d + 1, r));
            q.push_back((*v, d + 1, r));
        }
    }

    for (i, &s) in S.iter().enumerate() {
        if s == 'D' {
            let d: usize = dist[i].iter().map(|x| x.0).sum();
            println!("{d}");
        }
    }
}
