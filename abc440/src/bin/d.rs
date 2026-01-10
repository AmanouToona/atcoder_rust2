use proconio::input;
use std::collections::BTreeMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        mut A: [usize; N],
    }

    A.sort();
    let mut btree = BTreeMap::new();
    for (i, &a) in A.iter().enumerate() {
        btree.insert(a, i + 1);
    }

    for _ in 0..Q {
        input! {(X, Y): (usize, usize)}
        let (_, &background) = btree.range(0..X).next_back().unwrap_or((&0, &0));
        let mut right: usize = 10_000_000_000;

        let mut left = X;
        let (_, &u) = btree.range(0..=left).next_back().unwrap_or((&0, &0));
        if left - (X - 1) - (u - background) >= Y {
            // println!("... {left} ...");
            println!("{left}");
            continue;
        }

        while right - left > 1 {
            let mid = (right + left) / 2;
            let (_, &cant_use) = btree.range(0..=mid).next_back().unwrap_or((&0, &0));
            let cant_use = cant_use - background;
            if mid - (X - 1) - cant_use >= Y {
                right = mid;
            } else {
                left = mid;
            }
        }

        // println!("... {right} ...");
        println!("{right}");
    }
}
