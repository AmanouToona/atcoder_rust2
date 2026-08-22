#![allow(non_snake_case)]
use itertools::Itertools;
use num::Integer;
use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;

fn solve() {
    input! {
        (N, M): (usize, usize),
        ab: [(usize, usize); M],
    }
    let mut g = vec![Vec::new(); N];
    for &(a, b) in ab.iter() {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut parent = vec![usize::MAX; N];
    let mut depth = vec![usize::MAX; N];
    depth[0] = 0;

    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        for &v in g[u].iter() {
            if depth[v] == depth[u] {
                let mut s1 = HashSet::new();
                let mut s2 = HashSet::new();

                let mut r1 = Vec::new();
                let mut n1 = u;
                loop {
                    r1.push(n1);
                    n1 = parent[n1];
                    s1.insert(n1);
                    if n1 == usize::MAX {
                        break;
                    }
                }

                let mut r2 = Vec::new();
                let mut n2 = v;
                loop {
                    r2.push(n2);
                    n2 = parent[n2];
                    s2.insert(n2);
                    if n2 == usize::MAX {
                        break;
                    }
                }

                let mut ans = Vec::new();
                for n1 in r1.iter() {
                    ans.push(*n1);
                    if s2.contains(n1) {
                        break;
                    }
                }

                for n2 in r2.iter().rev() {
                    if s1.contains(n2) {
                        continue;
                    }
                    ans.push(*n2);
                }

                if ans.len().is_odd() {
                    println!("{}", ans.len());
                    println!("{}", ans.iter().map(|x| x + 1).join(" "));
                    return;
                }
            }

            if depth[v] != usize::MAX {
                continue;
            }

            parent[v] = u;
            depth[v] = depth[u] + 1;
            q.push_back(v);
        }
    }

    println!("-1");
}

fn main() {
    input! {T: usize}
    for _ in 0..T {
        solve();
    }
}
