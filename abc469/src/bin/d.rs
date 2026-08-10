#![allow(non_snake_case)]

use proconio::input;

/*
グラフとして考えて連結にする
2node のみで、グラフを連結すにする問題

*/
fn main() {
    input! {
        (N, M): (usize, usize),
        AB: [(usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];

    for &(a, b) in AB.iter() {
        g[a - 1].push(b);
        g[b - 1].push(a);
    }
}
　
