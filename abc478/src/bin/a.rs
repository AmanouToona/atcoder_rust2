#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut ans = vec![0; N];
    for i in 0..M {
        ans[i % N] += 1;
    }

    for &a in ans.iter() {
        println!("{a}");
    }
}
