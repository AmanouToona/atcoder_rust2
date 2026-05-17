#![allow(non_snake_case)]
use itertools::Itertools;
use num::Integer;
use proconio::input;
fn main() {
    input! {
        N: usize,
    }
    let mut ans = Vec::new();
    let mut n = N;

    while n > 0 {
        if n.is_odd() {
            n -= 1;
            ans.push('A');
        } else {
            n /= 2;
            ans.push('B');
        }
    }

    let ans: String = ans.iter().rev().join("");
    println!("{ans}");
}
