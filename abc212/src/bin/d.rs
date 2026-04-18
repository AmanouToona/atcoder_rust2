#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
fn main() {
    input! {
        Q: usize
    }

    let mut offset = 0i64;
    let mut q = BinaryHeap::new();
    for _ in 0..Q {
        input! {query: usize}
        match query {
            1 => {
                input! {x: i64}
                q.push((Reverse(x + offset), offset));
            }
            2 => {
                input! {x: i64}
                offset -= x;
            }
            3 => {
                if let Some((Reverse(x_offset), off)) = q.pop() {
                    let x = x_offset - off;
                    let ans = x - (offset - off);
                    println!("{ans}");
                }
            }
            _ => {
                panic!()
            }
        }
    }
}
