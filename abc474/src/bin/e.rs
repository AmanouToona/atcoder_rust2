#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
/*
同じ商品を複数回購入することが良いケースはあるか？
a, b = [(2, 1), (100, 1), (100, 1)] なら、 (2, 1) を 2 で 2回買って、クーポンを手にいれるのが特
では、このケースはどこで発生する?
クーポンを手にいれるためだけに買うのは、 a の最安値
クーポンを使いたいのは a - b のmax から
[(100, 0), (900, 700)] なら、 (100, 0) をaで購入し、 (900, 700) を bで購入
[(100, 0), (900, 899)] なら、 (900, 899) を a で購入し、 (100, 0) を 0　で購入
a1 の値段が、 ある　a - b よりやすいなら、 a1 を買った方がいい
*/
fn solve() {
    input! {
        N: usize,
        AB: [(usize, usize); N],
    }
    if N == 1 {
        println!("{}", AB[0].1);
        return;
    }

    let mut ab = AB.clone();
    ab.sort_by_key(|(a, b)| a - b);
    let a_min = ab.iter().map(|x| x.0).min().unwrap();

    // k 枚のクーポンを利用
    for k in 0..=N {}
}

fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        solve();
    }
}
