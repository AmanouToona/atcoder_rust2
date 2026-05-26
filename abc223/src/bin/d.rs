#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/*
グラフ?

トポロジカルソート
- 自分へ入ってくる次数 0 の vertex のうち、最小のものを利用

*/
fn main() {
    input! {
        (N, M):(usize, usize),
        ab: [(usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];
    let mut frm_cnt = vec![0; N];
    for &(a, b) in ab.iter() {
        g[a - 1].push(b - 1);
        frm_cnt[b - 1] += 1;
    }

    let mut vertex0: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for (i, frm) in frm_cnt.iter().enumerate() {
        if *frm == 0 {
            vertex0.push(Reverse(i));
        }
    }

    let mut ans = Vec::new();
    while let Some(Reverse(u)) = vertex0.pop() {
        ans.push(u);
        for &v in g[u].iter() {
            frm_cnt[v] -= 1;
            if frm_cnt[v] == 0 {
                vertex0.push(Reverse(v));
            }
        }
    }

    if ans.len() != N {
        println!("-1");
    } else {
        let ans: String = ans.iter().map(|i| *i + 1).join(" ");
        println!("{ans}");
    }
}
