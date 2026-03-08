#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        A: [usize; N],
    }

    let mut A: Vec<(usize, usize)> = A.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
    A.sort();

    for _ in 0..Q {
        input! {K: usize,
            B: [usize; K]
        }

        let B: HashSet<usize> =
            std::collections::HashSet::from_iter(B.iter().cloned().map(|x| x - 1));

        for &(a, i) in A.iter() {
            if B.contains(&i) {
                continue;
            } else {
                println!("{a}");
                break;
            }
        }
    }
}
