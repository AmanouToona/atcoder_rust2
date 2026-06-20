#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {X: usize}
    let S: String = "HelloWorld".to_string();
    let ans = S
        .chars()
        .enumerate()
        .filter(|x| x.0 != X - 1)
        .map(|x| x.1)
        .join("");
    println!("{ans}");
}
