#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }

    for (i, &a) in A.iter().enumerate() {
        if B[a - 1] != i + 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
