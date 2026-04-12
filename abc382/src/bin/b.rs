#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (_, D): (usize, usize),
        mut S: Chars,
    }

    let mut res = D;
    for s in S.iter_mut().rev() {
        if *s == '@' {
            *s = '.';
            res -= 1;
        }
        if res == 0 {
            break;
        }
    }
    println!("{}", S.iter().join(""));
}
