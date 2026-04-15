#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn dfs(u: usize, seen: &mut Vec<bool>, g: &Vec<Vec<usize>>, ans: &mut Vec<usize>) {
    ans.push(u);
    for v in g[u].iter() {
        if seen[*v] {
            continue;
        }
        seen[*v] = true;
        dfs(*v, seen, g, ans);
        ans.push(u);
    }
}
fn main() {
    input! {
        N: usize,
        ab: [(usize, usize); N - 1],
    }

    let mut g = vec![Vec::new(); N];
    for &(a, b) in ab.iter() {
        let a = a - 1;
        let b = b - 1;
        g[a].push(b);
        g[b].push(a);
    }
    for i in g.iter_mut() {
        i.sort();
    }

    // オイラーツアー みたいな感じ
    let mut ans = Vec::new();
    let mut seen = vec![false; N];
    seen[0] = true;
    dfs(0, &mut seen, &g, &mut ans);
    let ans: String = ans.iter().map(|x| *x + 1).join(" ");
    println!("{ans}");
}
