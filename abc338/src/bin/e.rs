#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N],
    }

    let mut AB: Vec<(usize, usize)> = AB
        .into_iter()
        .map(|(a, b)| {
            if a < b {
                (a - 1, b - 1)
            } else {
                (b - 1, a - 1)
            }
        })
        .collect();
    AB.sort();

    let mut q: VecDeque<usize> = VecDeque::new();

    for &(a, b) in AB.iter() {
        while let Some(&cap) = q.back() {
            if cap < a {
                q.pop_back();
            } else {
                break;
            }
        }

        if let Some(&cap) = q.back() {
            if cap < b {
                println!("Yes");
                return;
            }
        }

        q.push_back(b);
    }
    println!("No");
    // eprintln!("{:?}", AB);
}
