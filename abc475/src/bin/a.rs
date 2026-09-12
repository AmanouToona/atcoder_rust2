#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut ans = Vec::new();
    for (i, c) in S.iter().enumerate() {
        if i != 0 {
            ans.push('o');
        };
        ans.push(*c);
    }
    let ans: String = ans.iter().join("");
    println!("{ans}");
}
