#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, X): (usize, usize),
        A: [usize; N],
    }

    let mut X = X;
    for &a in A.iter() {
        if a >= X {
            println!("0");
        } else {
            X = a;
            println!("1");
        }
    }
}
