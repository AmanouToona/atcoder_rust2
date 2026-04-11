#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let mut ans = Vec::new();
    for &s in S.iter() {
        if s == 'o' && ans.is_empty() {
            continue;
        }
        ans.push(s);
    }

    let ans: String = ans.iter().join("");
    println!("{ans}");
}
