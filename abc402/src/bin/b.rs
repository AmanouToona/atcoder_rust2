#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        Q: usize,
    }

    let mut q = VecDeque::new();
    for _ in 0..Q {
        input! {query: usize}
        match query {
            1 => {
                input! {x: usize}
                q.push_back(x);
            }
            2 => {
                if let Some(x) = q.pop_front() {
                    println!("{x}");
                }
            }
            _ => {
                panic!("wrong")
            }
        }
    }
}
