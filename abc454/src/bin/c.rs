#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
/*
グラフっぽい
a -> b の有効編がある。
辿り着ける場所の数は？
*/
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
    }

    let mut reach = vec![false; N];
    let mut q = VecDeque::new();
    reach[0] = true;
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        for &v in g[u].iter() {
            if !reach[v] {
                reach[v] = true;
                q.push_back(v);
            }
        }
    }

    let mut ans = 0;
    for i in reach.iter() {
        if *i {
            ans += 1;
        }
    }

    println!("{ans}");
}
