#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        ab: [(usize, usize); N],
    }

    let mut cnt = vec![0; M];
    for &(a, b) in ab.iter() {
        let a = a - 1;
        let b = b - 1;

        cnt[a] -= 1;
        cnt[b] += 1;
    }

    for &ans in cnt.iter() {
        println!("{ans}");
    }
}
