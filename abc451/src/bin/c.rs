#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
fn main() {
    input! {
        Q: usize,
    }

    let mut tree: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for _ in 0..Q {
        input! {q: usize, h: usize}
        match q {
            1 => {
                tree.push(Reverse(h));
            }
            2 => {
                while let Some(&Reverse(t)) = tree.peek() {
                    if t > h {
                        break;
                    }
                    tree.pop();
                }
            }
            _ => {
                panic!()
            }
        }

        println!("{}", tree.len());
    }
}
