#![allow(non_snake_case)]
use std::{eprintln, println};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let mut count_x = vec![0; N + 1];
    for (i, &c) in S.iter().enumerate() {
        if c == 'x' {
            count_x[i + 1] = count_x[i] + 1;
        } else {
            count_x[i + 1] = count_x[i];
        }
    }

    eprintln!("{:?}", count_x);
}
