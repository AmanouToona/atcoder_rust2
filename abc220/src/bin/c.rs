#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [i64; N],
        mut X: i64,
    }

    let sum_a: i64 = A.iter().sum();
    let mut ans = (X / sum_a) as usize * N;
    X %= sum_a;

    let mut iter = A.into_iter();
    while X >= 0 {
        ans += 1;
        X -= iter.next().unwrap();
    }
    println!("{ans}");
}
