#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {T: usize}
    for _ in 0..T {
        input! {
            N: usize,
            A: [usize; 2*N],
        }

        let mut ans = Vec::new();
        let mut set: HashSet<usize> = HashSet::new();
        let mut trash: HashSet<usize> = HashSet::new();
        let mut take = true;
        for (&a, i) in A.iter().zip(0..) {
            if take {
                if set.contains(&a) {
                    trash.insert(a);
                    ans.push(i);
                    take = false;
                } else {
                    set.insert(a);
                }
            } else {
                if trash.contains(&a) {
                    set.insert(a);
                    ans.push(i);
                    take = true;
                } else {
                    trash.insert(a);
                }
            }
        }
        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
    }
}
