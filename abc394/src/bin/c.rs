#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut ans = S.clone();
    let mut convert_c = vec![false; S.len()];
    for i in (1..S.len()).rev() {
        if ans[i] == 'A' && ans[i - 1] == 'W' {
            ans.swap(i, i - 1);
            convert_c[i] = true;
            convert_c[i - 1] = true;
        }
    }

    for i in 0..S.len() {
        if convert_c[i] && ans[i] == 'W' {
            ans[i] = 'C';
        }
    }

    let ans: String = ans.iter().join("");
    println!("{ans}");
}
