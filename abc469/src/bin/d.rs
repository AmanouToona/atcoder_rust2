#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        (N, M): (usize, usize),
        ab: [(usize, usize); M],
    }
    let x = ab[0].0;
    let y = ab[0].1;

    let mut set1: HashSet<usize> = HashSet::from_iter(1..=N);
    set1.remove(&x);
    let mut set2: HashSet<usize> = HashSet::from_iter(1..=N);
    set2.remove(&x);
    set2.remove(&y);

    for &(a, b) in ab.iter().skip(1) {
        // 1
        if a != x && b != x {
            set1 = set1
                .intersection(&(HashSet::from([a, b])))
                .cloned()
                .collect();
        }
        // 2
        if a != y && b != y {
            set2 = set2
                .intersection(&(HashSet::from([a, b])))
                .cloned()
                .collect();
        }
    }

    println!("{}", set1.len() + set2.len());
}
