#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn dfs(
    u: usize,
    p: usize,
    g: &[Vec<usize>],
    a: &[usize],
    seen: &mut HashMap<usize, usize>,
    twice: &mut HashSet<usize>,
    ans: &mut [bool],
) {
    // eprintln!("{} {:?} {:?} {:?}", u, p, seen, twice);
    *seen.entry(a[u]).or_default() += 1;

    if seen.get(&a[u]).unwrap() >= &2 {
        twice.insert(a[u]);
    }
    if !twice.is_empty() {
        ans[u] = true;
    }

    for &v in g[u].iter() {
        if v == p {
            continue;
        }

        dfs(v, u, g, a, seen, twice, ans);
    }

    *seen.entry(a[u]).or_default() -= 1;
    if seen.get(&a[u]).unwrap() < &2 {
        twice.remove(&a[u]);
    }
}

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        UV: [(usize, usize); N - 1],
    }

    let mut g = vec![Vec::new(); N];
    for &(u, v) in UV.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push(v);
        g[v].push(u);
    }

    let mut ans = vec![false; N];
    let mut seen = HashMap::new();
    let mut twice = HashSet::new();

    dfs(0, !0, &g, &A, &mut seen, &mut twice, &mut ans);

    for &i in ans.iter() {
        if i {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
