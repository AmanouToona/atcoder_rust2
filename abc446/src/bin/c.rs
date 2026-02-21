#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            (N, D): (usize, usize),
            A: [usize; N],
            B: [usize; N],
        }

        let mut q = VecDeque::new();
        for (i, (&a, &b)) in A.iter().zip(B.iter()).enumerate() {
            for _ in 0..a {
                q.push_back(i);
            }
            for _ in 0..b {
                q.pop_front();
            }
            while let Some(u) = q.front() {
                if i >= u + D {
                    q.pop_front();
                } else {
                    break;
                }
            }
        }
        println!("{}", q.len());
    }
}
