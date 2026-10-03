#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        (N, Q): (usize, usize),
        LRX: [(usize, usize, usize); Q],
    }

    let mut ope_add = vec![Vec::new(); N + 1];
    let mut ope_sub = vec![Vec::new(); N + 1];

    for &(l, r, x) in LRX.iter() {
        ope_add[l - 1].push(x);
        ope_sub[r].push(x);
    }

    let mut now: HashMap<usize, usize> = HashMap::new();
    let mut ans = Vec::new();

    for i in 0..N {
        for &j in ope_add[i].iter() {
            *now.entry(j).or_default() += 1;
        }
        for &j in ope_sub[i].iter() {
            *now.entry(j).or_default() -= 1;
            if now[&j] == 0 {
                now.remove(&j);
            }
        }
        ans.push(now.len());
    }

    println!("{}", ans.iter().join(" "));
}
