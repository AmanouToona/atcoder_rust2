#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut ans = Vec::new();
    for (i, &s) in S.iter().enumerate() {
        if s == '#' {
            ans.push(i + 1);
        }
        if ans.len() == 2 {
            let output: String = ans.iter().join(",");
            println!("{output}");
            ans = Vec::new();
        }
    }
}
