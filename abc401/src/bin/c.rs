#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        (N, K): (usize, usize)
    }

    let m = 10usize.pow(9);

    if N < K {
        println!("1");
        return;
    }

    let mut q = VecDeque::new();
    for _ in 0..K {
        q.push_back(1);
    }
    let mut cumsum = K;

    for _ in K..=N {
        cumsum %= m;
        q.push_back(cumsum);
        cumsum *= 2;
        cumsum = cumsum + m - q.pop_front().unwrap();
    }
    println!("{}", q.back().unwrap());
}
