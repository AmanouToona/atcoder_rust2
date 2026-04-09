#![allow(non_snake_case)]

use proconio::input;

fn dfs(g: &Vec<Vec<usize>>, u: usize, p: usize, K: usize) -> Option<usize> {
    let mut paths: Vec<usize> = Vec::new();
    for &v in g[u].iter() {
        if v == p {
            continue;
        }
        let l = dfs(g, v, u, K)?;
        if l > 0 {
            paths.push(l);
        }
    }

    let res = paths.iter().sum::<usize>() + 1;

    match paths.len() {
        1 | 0 => {
            if res == K {
                Some(0)
            } else {
                Some(res)
            }
        }
        2 => {
            if res == K {
                Some(0)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn main() {
    input! {
        (N, K): (usize, usize),
        uv: [(usize, usize); N * K - 1],
    }

    let mut g = vec![Vec::new(); N * K];
    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push(v);
        g[v].push(u);
    }

    if Some(0) == dfs(&g, 0, 0, K) {
        println!("Yes");
    } else {
        println!("No");
    }
}
