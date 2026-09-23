#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }

    for (i, p) in P.iter().enumerate() {
        let n = i / 10 + 1;
        if n != p.div_ceil(10) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
