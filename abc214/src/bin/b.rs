#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (S, T) : (usize, usize),
    }

    let mut ans = 0;
    for a in 0..=100 {
        for b in 0..=100 {
            for c in 0..=100 {
                if a + b + c <= S && a * b * c <= T {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}
