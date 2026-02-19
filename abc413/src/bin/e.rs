#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        T: usize
    }
    for _ in 0..T {
        input! {
            N: usize,
            mut P: [usize; 2_i32.pow(N as u32)],
        }

        let mut w = 1;
        while w <= (1 << N) / 2 {
            for l in (0..(1 << N) - w).step_by(w * 2) {
                if P[l] > P[l + w] {
                    for i in 0..w {
                        P.swap(l + i, l + w + i);
                    }
                }
            }
            w <<= 1;
        }
        let ans: String = P.iter().join(" ");
        println!("{ans}");
    }
}
