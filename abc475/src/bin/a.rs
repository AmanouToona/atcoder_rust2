#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut ans = Vec::new();
    for i in 0..S.len() {
        if i == 0 {
            ans.push(S[i]);
        } else {
            ans.push('o');
            ans.push(S[i]);
        }
    }
    let ans: String = ans.iter().join("");
    println!("{ans}");
}
