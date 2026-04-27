#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize
    }

    let mut k = 0;
    while 2usize.pow(k + 1) <= N {
        k += 1;
    }

    println!("{k}");
}
