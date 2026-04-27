#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (A, B, C): (usize, usize, usize)
    }

    for c in A..=B {
        if c >= C && c % C == 0 {
            println!("{c}");
            return;
        }
    }
    println!("-1");
}
