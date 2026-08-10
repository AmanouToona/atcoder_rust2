#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        cs: [(usize, i64); N],
    }

    let mut largest = vec![-1; M];
    for &(c, s) in cs.iter() {
        largest[c - 1] = largest[c - 1].max(s);
    }

    let ans: String = largest.iter().join(" ");
    println!("{ans}");
}
