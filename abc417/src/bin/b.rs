#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        mut A: [usize; N],
        B: [usize; M],
    }

    for &b in B.iter() {
        for a in A.iter_mut() {
            if b == *a {
                *a = usize::MAX;
                break;
            }
        }
    }

    let ans: String = A.iter().filter(|&x| *x != usize::MAX).join(" ");
    if !ans.is_empty() {
        println!("{ans}");
    }
}
