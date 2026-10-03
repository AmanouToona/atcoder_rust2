#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut heap = BinaryHeap::new();

    for &a in A.iter().take(2) {
        heap.push(Reverse(a));
    }

    for &a in A.iter().skip(2) {
        heap.push(Reverse(a));

        if heap.len() > 3 {
            heap.pop();
        }

        println!("{}", heap.peek().unwrap().0);
    }
}
