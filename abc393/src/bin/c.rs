#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (_, M) : (usize, usize),
        uv: [(usize, usize); M],
    }

    let mut ans = 0;
    let mut seen = HashSet::new();
    for &(u, v) in uv.iter() {
        if seen.contains(&(u, v)) {
            ans += 1;
            continue;
        };

        if u == v {
            ans += 1;
            continue;
        }

        seen.insert((u, v));
        seen.insert((v, u));
    }

    println!("{ans}");
}
