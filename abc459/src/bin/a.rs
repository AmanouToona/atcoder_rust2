#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        X: usize,
    }

    let S: String = "HelloWorld".to_string();
    let mut ans = Vec::new();
    for (i, s) in S.chars().enumerate() {
        if i != X - 1 {
            ans.push(s);
        }
    }
    let ans: String = ans.iter().join("");
    println!("{ans}");
}
