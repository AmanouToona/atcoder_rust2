#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        (N, K): (usize, usize),
         A: [usize; N],
    }

    let a_max = *A.iter().max().unwrap();

    let mut A: Vec<usize> = A.iter().map(|&a| a + (a_max - a) / K * K).collect();
    A.sort();

    let mut normal = VecDeque::from_iter(A.iter().cloned());
    let mut ans = normal[N - 1] - normal[0];
    for _ in 0..N {
        let a = normal.pop_front().unwrap();
        normal.push_back(a + K);
        ans = ans.min(normal[N - 1] - normal[0]);
    }

    println!("{ans}");
}
