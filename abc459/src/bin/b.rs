#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }

    let f = |s: char| match s as u8 - 'a' as u8 {
        0..=2 => '2',
        3..=5 => '3',
        6..=8 => '4',
        9..=11 => '5',
        12..=14 => '6',
        15..=18 => '7',
        19..=21 => '8',
        22..=25 => '9',
        _ => {
            panic!()
        }
    };

    let ans = S.iter().map(|x| f(x[0])).join("");
    println!("{ans}");
}
