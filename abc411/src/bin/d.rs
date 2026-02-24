#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (N, Q): (usize, usize),
    }

    let mut from = vec![usize::MAX]; // max is sentinel
    let mut node: Vec<Vec<char>> = vec!["".chars().collect()];
    let mut pc: Vec<usize> = vec![0; N + 1];

    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                input! {p: usize}
                pc[p] = pc[0];
            }
            2 => {
                input! {p: usize, s: Chars}
                let new_idx = node.len();
                node.push(s);
                from.push(pc[p]);
                pc[p] = new_idx;
            }
            3 => {
                input! {p: usize}
                pc[0] = pc[p];
            }
            _ => {
                panic!("{q}")
            }
        }
    }

    let mut ans = Vec::new();
    let mut n = pc[0];
    while n != usize::MAX {
        ans.push(node[n].clone());
        n = from[n];
    }
    let ans: String = ans.iter().rev().flatten().join("");
    println!("{ans}");
}
