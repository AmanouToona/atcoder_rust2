#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        cs: [(usize, i64); N],
    }

    let mut color_large = vec![-1; M];
    for &(c, s) in cs.iter() {
        color_large[c - 1] = color_large[c - 1].max(s);
    }

    let ans: String = color_large.iter().join(" ");
    println!("{ans}");
}
