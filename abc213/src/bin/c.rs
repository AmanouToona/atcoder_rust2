#![allow(non_snake_case)]
use proconio::input;
use std::collections::{BTreeSet, HashMap};
fn main() {
    input! {
        (_, _, N): (usize, usize, usize),
        ab: [(usize, usize); N],
    }

    /*
    数が入っている、列、行を抜き出す。
    座標圧縮 (元の行, 圧縮後の行) を作る
     */

    let mut a_s = BTreeSet::from_iter(ab.iter().map(|x| x.0));
    let mut a_press = HashMap::new();
    while let Some(i) = a_s.pop_first() {
        a_press.insert(i, a_press.len());
    }

    let mut b_s = BTreeSet::from_iter(ab.iter().map(|x| x.1));
    let mut b_press = HashMap::new();
    while let Some(i) = b_s.pop_first() {
        b_press.insert(i, b_press.len());
    }

    for &(a, b) in ab.iter() {
        println!("{} {}", a_press[&a] + 1, b_press[&b] + 1);
    }
}
