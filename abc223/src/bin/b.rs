#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut ss = Vec::new();
    for i in 0..S.len() {
        let s = S.iter().cycle().skip(i).take(S.len()).join("");
        ss.push(s);
    }

    ss.sort();
    println!("{}", ss[0]);
    println!("{}", ss[S.len() - 1]);
}
