#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }

    let mut res: i128 = 0; // if res > 0 then there is ans
    let mut W: Vec<usize> = Vec::new();
    for (&a, &b) in A.iter().zip(B.iter()) {
        if a <= b {
            W.push(1);
            res -= (b - a) as i128;
        } else {
            W.push(1_000_000_000_000_000_000);
            res += (a - b) as i128 * 1_000_000_000_000_000_000;
        }
    }

    if res > 0 {
        println!("Yes");
        println!("{}", W.iter().join(" "));
    } else {
        println!("No");
    }
}
