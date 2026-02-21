#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        X: usize
    }

    for &a in A.iter() {
        if a == X {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
