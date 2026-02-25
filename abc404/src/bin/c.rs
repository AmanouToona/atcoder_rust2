#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        ab: [(usize, usize); M],
    }
    let mut g = vec![Vec::new(); N];
    for &(a, b) in ab.iter() {
        let a = a - 1;
        let b = b - 1;
        g[a].push(b);
        g[b].push(a);
    }

    if g.iter().any(|x| x.len() != 2) {
        println!("No");
        return;
    }

    let mut seen = vec![false; N];
    let mut u = 0;
    let mut p = N;
    loop {
        if seen[u] {
            break;
        }

        //更新
        seen[u] = true;
        for &v in g[u].iter() {
            if v == p {
                continue;
            } else {
                p = u;
                u = v;
                break;
            }
        }
    }

    if seen.iter().all(|x| *x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
