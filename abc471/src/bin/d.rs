#![allow(non_snake_case)]
use proconio::input;
use std::collections::BinaryHeap;
fn main() {
    input! {
        (Q, V): (usize, i64),
    }

    let mut q = BinaryHeap::new();
    for _ in 0..Q {
        input! {query: usize}
        match query {
            1 => {
                input! {(t, w): (i64, i64)}
                q.push(w - t);
            }
            _ => {
                input! {t: i64}
                if q.is_empty() {
                    println!("-1");
                } else {
                    let ans = (q.pop().unwrap() + t).min(V);
                    println!("{ans}");
                }
            }
        }
    }
}
