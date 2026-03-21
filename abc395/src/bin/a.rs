#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    for (&a1, &a2) in A.iter().zip(A.iter().skip(1)) {
        if a1 >= a2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
