#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let mut ans = vec![0; N];
    let mut l = 0;
    let mut r = N - 1;

    let mut use_r = true;
    for (i, &c) in S.iter().enumerate().rev() {
        if c == 'o' {
            use_r = !use_r;
        }
        if use_r {
            ans[r] = i + 1;
            r -= 1;
        } else {
            ans[l] = i + 1;
            l += 1;
        }
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
