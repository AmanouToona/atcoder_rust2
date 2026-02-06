#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            N: usize,
            S: Chars,
        }

        let mut now0 = 0;
        let mut len0 = 0;
        let mut now1 = 0;
        let mut len1 = 0;

        for &s in S.iter() {
            if s == '0' {
                now0 += 1;
                now1 = 0;
            } else {
                now0 = 0;
                now1 += 1;
            }
            len0 = len0.max(now0);
            len1 = len1.max(now1);
        }

        let mut sum1 = 0;
        let mut sum0 = 0;
        for &s in S.iter() {
            if s == '0' {
                sum0 += 2;
                sum1 += 1;
            } else {
                sum0 += 1;
                sum1 += 2;
            }
        }

        let ans = (sum1 - len1 * 2).min(sum0 - len0 * 2);
        println!("{ans}");
    }
}
