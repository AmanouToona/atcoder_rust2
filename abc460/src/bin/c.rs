#![allow(non_snake_case)]
use proconio::input;

/*
b < a * 2 が必要

大きなaから始めて小さくしていく
*/
fn main() {
    input! {
        (N, M): (usize, usize),
        mut A: [usize; N],
        mut B: [usize; M],
    }

    A.sort_by(|x, y| y.cmp(x));
    B.sort_by(|x, y| y.cmp(x));

    let mut ans = 0;
    let mut i = 0;
    for &a in A.iter() {
        while i < M {
            if B[i] <= a * 2 {
                break;
            } else {
                i += 1;
            }
        }

        if i >= M {
            break;
        }
        i += 1;
        ans += 1;
    }
    println!("{ans}")
}
