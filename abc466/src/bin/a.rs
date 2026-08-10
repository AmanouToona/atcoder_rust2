#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        X: [i64; N],
    }

    if X.iter().max().unwrap() < &0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
