#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        N: usize,
        D: [usize; N - 1],
    }

    for i in 0..N - 1 {
        let mut sum = 0;
        let mut ans = Vec::new();
        for &d in D[i..N - 1].iter() {
            sum += d;
            ans.push(sum);
        }
        let ans: String = ans.iter().join(" ");
        println!("{ans}");
    }
}
