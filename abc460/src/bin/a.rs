#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut m = M;
    let mut ans = 0;
    while m != 0 {
        let x = N % m;
        m = x;
        ans += 1;
    }
    println!("{ans}");
}
