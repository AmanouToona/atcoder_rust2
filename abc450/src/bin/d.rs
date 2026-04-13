#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        (N, K): (usize, usize),
         mut A: [usize; N],
    }

    A = A.iter().map(|&x| x % K).collect();
    A.sort();

    let mut q: VecDeque<usize> = VecDeque::from_iter(A.iter().cloned());
    let mut ans = A.last().unwrap() - A[0];
    for _ in 0..N {
        ans = ans.min(q.back().unwrap() - q.front().unwrap());
        let front = q.pop_front().unwrap();
        q.push_back(front + K);
    }

    println!("{ans}");
}
