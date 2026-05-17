#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
    }

    let mut A = Vec::new();
    for _ in 0..N {
        input! {L: usize, a: [usize; L]}
        A.push(a);
    }
    input! {(X, Y): (usize, usize)}

    println!("{}", A[X - 1][Y - 1]);
}
