#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        K: usize,
        (A, B): (Chars, Chars),
    }

    let mut a10 = 0;
    for (i, a) in A.iter().rev().enumerate() {
        a10 += a.to_digit(10).unwrap() as usize * K.pow(i as u32);
    }

    eprintln!("{a10}");

    let mut b10 = 0;
    for (i, b) in B.iter().rev().enumerate() {
        b10 += b.to_digit(10).unwrap() as usize * K.pow(i as u32);
    }

    println!("{}", a10 * b10);
}
