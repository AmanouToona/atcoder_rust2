#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
    }

    if A.iter().sum::<usize>() <= M {
        println!("Yes");
    } else {
        println!("No");
    }
}
