#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        N: usize,
        S: [i64; N],
    }

    let mut sum = 0;
    let mut a = Vec::new();

    for s in S.iter() {
        a.push(s - sum);
        sum += a.last().unwrap();
    }

    println!("{}", a.iter().join(" "));
}
