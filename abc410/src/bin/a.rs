#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        K: usize,
    }

    let ans = A.iter().map(|&x| if x >= K { 1 } else { 0 }).sum::<usize>();
    println!("{ans}");
}
