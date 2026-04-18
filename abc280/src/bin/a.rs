#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (H, _): (usize, usize),
        S: [Chars; H],
    }

    let mut ans = 0;
    for s in S.iter() {
        for ss in s.iter() {
            if *ss == '#' {
                ans += 1;
            }
        }
    }

    println!("{ans}")
}
