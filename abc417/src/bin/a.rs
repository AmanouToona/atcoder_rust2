#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (N, A, B): (usize, usize, usize),
        S: Chars,
    }

    let ans = S.iter().skip(A).take(N - A - B).collect::<String>();
    println!("{ans}");
}
