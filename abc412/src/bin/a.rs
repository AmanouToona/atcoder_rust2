#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        n: usize,
        AB: [(usize, usize); n],
    }

    let ans: usize = AB.iter().map(|&x| if x.1 > x.0 { 1 } else { 0 }).sum();
    println!("{ans}");
}
