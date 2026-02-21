#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
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

    let mut q =

}
