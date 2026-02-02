#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn dfs(
    g: &Vec<Vec<usize>>,
    from: usize,
    to: usize,
    q: &mut Vec<usize>,
    ans: &mut Vec<usize>,
    used: &mut Vec<bool>,
) {
    if !ans.is_empty() {
        return;
    }
    if from == to {
        *ans = (*q).clone();
        return;
    }
    for &nxt in g[from].iter() {
        if used[nxt] {
            continue;
        }
        q.push(nxt);
        used[nxt] = true;
        dfs(g, nxt, to, q, ans, used);
        q.pop();
    }
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

        let mut ans = Vec::new();
        let mut q = Vec::new();
        let mut used = vec![false; N];
        q.push(X - 1);
        used[X - 1] = true;
        dfs(&g, X - 1, Y - 1, &mut q, &mut ans, &mut used);

        let ans: String = ans.iter().map(|x| *x + 1).join(" ");
        println!("{ans}");
    }
}
