#![allow(non_snake_case)]
use itertools::{repeat_n, Itertools};
use proconio::input;
fn main() {
    input! {
        N: usize,
        cl: [(char, i128); N],
    }

    if cl.iter().map(|(_, l)| *l).sum::<i128>() > 100 {
        println!("Too Long");
        return;
    }

    let ans: String = cl
        .iter()
        .flat_map(|&(c, l)| repeat_n(c, l as usize))
        .join("");
    println!("{ans}");
}
