#![allow(non_snake_case)]
use proconio::input;
use std::collections::BTreeSet;
/*
二分木に切った一を持たせる。

端点の処理が必要
端点は, 0, L で切られているいると解釈できる

*/

fn main() {
    input! {
        (L, Q): (usize, usize),
    }

    let mut cut = BTreeSet::new();
    cut.insert(0);
    cut.insert(L);
    for _ in 0..Q {
        input! {(c, x): (usize, usize)}

        match c {
            1 => {
                cut.insert(x);
            }
            2 => {
                let ans = cut.range(x..).next().unwrap() - cut.range(0..x).next_back().unwrap();
                println!("{ans}");
            }
            _ => {}
        }
    }
}
