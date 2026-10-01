#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut tiled = HashSet::new();
    let mut non_tiled: HashSet<usize> = HashSet::from_iter(0..N);

    let mut query = Vec::new();
    for _ in 0..Q {
        input! {(q, c): (usize, String)};
        if q == 1 {
            let n = c.parse::<usize>().unwrap() - 1;
            if tiled.contains(&n) {
                non_tiled.insert(n);
                tiled.remove(&n);
            } else {
                tiled.insert(n);
                non_tiled.remove(&n);
            }
        }
        query.push((q, c));
    }

    let mut ans = vec!['a'; N];
    for (q, c) in query.into_iter().rev() {
        match q {
            1 => {
                let n = c.parse::<usize>().unwrap() - 1;
                if tiled.contains(&n) {
                    tiled.remove(&n);
                    non_tiled.insert(n);
                } else if non_tiled.contains(&n) {
                    non_tiled.remove(&n);
                    tiled.insert(n);
                }
            }
            2 => {
                for n in non_tiled.iter() {
                    ans[*n] = c.chars().next().unwrap();
                }
                non_tiled = HashSet::new();
            }
            _ => {
                panic!()
            }
        }
    }
    let ans: String = ans.iter().join("");
    println!("{ans}");
}
