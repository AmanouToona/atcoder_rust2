#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (H, W): (usize, usize),
        A: [[usize; W]; H],
    }

    for i1 in 0..H {
        for i2 in i1 + 1..H {
            for j1 in 0..W {
                for j2 in j1 + 1..W {
                    if A[i1][j1] + A[i2][j2] > A[i2][j1] + A[i1][j2] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
