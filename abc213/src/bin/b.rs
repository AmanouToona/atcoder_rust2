#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut A: Vec<(usize, usize)> = A.iter().enumerate().map(|(i, &a)| (a, i)).collect();
    A.sort();

    println!("{}", A[A.len() - 2].1 + 1);
}
