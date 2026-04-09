#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        uv: [(usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];
    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push(v);
    }

    let mut reach = std::collections::HashSet::new();
    let mut edge = std::collections::BTreeSet::new();

    reach.insert(0);
    for &e in g[0].iter() {
        edge.insert(e);
    }

    for i in 0..N {
        while let Some(&u) = edge.first() {
            if u > i {
                break;
            }

            if reach.contains(&u) {
                edge.pop_first();
                continue;
            }
            reach.insert(u);

            for &v in g[u].iter() {
                if reach.contains(&v) {
                    continue;
                }
                edge.insert(v);
            }
        }

        if reach.len() != i + 1 {
            println!("-1");
        } else {
            println!("{}", edge.len());
        }
    }
}
