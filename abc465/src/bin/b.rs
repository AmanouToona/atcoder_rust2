#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (X, Y, L, R, A, B): (usize, usize, usize, usize, usize, usize)
    }

    let mut ans = 0;
    for t in A..B {
        if t >= L && t < R {
            ans += X;
        } else {
            ans += Y;
        }
    }

    println!("{ans}");
}
