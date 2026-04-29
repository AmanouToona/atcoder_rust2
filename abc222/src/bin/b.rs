#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, P): (usize, usize),
        a: [usize; N],
    }

    let ans = a.iter().filter(|x| **x < P).count();
    println!("{ans}");
}
