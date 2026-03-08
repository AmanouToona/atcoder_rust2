#![allow(non_snake_case)]
use ac_library::Dsu;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (N, M):( usize, usize),
        uv: [(usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];
    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push(v);
        g[v].push(u);
    }

    let mut to_remove = HashSet::new();
    let mut connected = Dsu::new(N);
    for i in 0..N {
        for &v in g[i].iter() {
            if v > i {
                to_remove.insert(v);
                continue;
            }
            connected.merge(v, i);
        }

        to_remove.remove(&i);
        if connected.size(0) == i + 1 {
            println!("{}", to_remove.len());
        } else {
            println!("-1");
        }
    }
}
