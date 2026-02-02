#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn dfs(
    u: usize,
    goal: usize,
    seen: &mut Vec<bool>,
    path: &mut Vec<usize>,
    g: &Vec<Vec<usize>>,
) -> bool {
    path.push(u);
    seen[u] = true;

    if u == goal {
        return true;
    }

    for &nxt in g[u].iter() {
        if seen[nxt] {
            continue;
        }
        if dfs(nxt, goal, seen, path, g) {
            return true;
        }
    }
    path.pop();
    false
}

fn main() {
    input! {T: usize}
    for _ in 0..T {
        input! {
            (N, M, X, Y): (usize, usize ,usize, usize),
            uv: [(usize, usize); M],
        }

        let mut g: Vec<Vec<usize>> = vec![Vec::new(); N];
        for &(u, v) in uv.iter() {
            g[u - 1].push(v - 1);
            g[v - 1].push(u - 1);
        }

        for i in g.iter_mut() {
            i.sort();
        }

        let mut path = Vec::new();
        let mut seen = vec![false; N];

        dfs(X - 1, Y - 1, &mut seen, &mut path, &g);

        let ans: String = path.iter().map(|x| *x + 1).join(" ");
        println!("{ans}");
    }
}
