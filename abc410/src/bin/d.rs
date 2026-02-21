#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        abw: [(usize, usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];
    for &(a, b, w) in abw.iter() {
        let a = a - 1;
        let b = b - 1;
        g[a].push((b, w));
    }

    let mut q = vec![(0, 0)];
    let mut cost = vec![vec![false; 2 << 10]; N];
    cost[0][0] = true;
    while let Some((u, c_u)) = q.pop() {
        for &(v, c) in g[u].iter() {
            let c_nxt = c_u ^ c;
            if cost[v][c_nxt] {
                continue;
            }

            cost[v][c_nxt] = true;
            q.push((v, c_nxt));
        }
    }

    if let Some(ans) = cost[N - 1].iter().position(|&x| x) {
        println!("{ans}");
    } else {
        println!("-1");
    }
}
