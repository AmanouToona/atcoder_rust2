#![allow(non_snake_case)]
use proconio::input;
use std::collections::BinaryHeap;
use std::collections::HashMap;
fn main() {
    input! {
        (N, K, M): (usize, usize, usize),
        cv: [(usize, usize); N]
    }

    let mut color_num: HashMap<usize, usize> = HashMap::new();

    for &(c, _) in cv.iter() {
        if color_num.contains_key(&c) {
            continue;
        }
        color_num.insert(c, color_num.len());
    }

    let mut gems = vec![BinaryHeap::new(); color_num.len()];
    for &(c, v) in cv.iter() {
        gems[color_num[&c]].push(v);
    }

    gems.sort_by(|x, y| y.peek().unwrap().cmp(x.peek().unwrap()));

    let mut ans = 0;
    for m in 0..M {
        ans += gems[m].pop().unwrap();
    }

    let mut res = Vec::new();
    for i in gems.iter() {
        for j in i.iter() {
            res.push(*j);
        }
    }

    res.sort_by(|x, y| y.cmp(x));

    for i in 0..K - M {
        ans += res[i]
    }

    println!("{ans}")
}
