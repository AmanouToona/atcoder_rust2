#![allow(non_snake_case)]
use proconio::input;
use std::mem;
fn main() {
    input! {
        T: usize
    }
    for _ in 0..T {
        input! {
             (mut x1, mut y1, mut r1, mut x2, mut y2, mut r2): (i128, i128, i128, i128, i128, i128),
        }

        if r1 < r2 {
            mem::swap(&mut x1, &mut x2);
            mem::swap(&mut y1, &mut y2);
            mem::swap(&mut r1, &mut r2);
        }

        let dpow = (x1 - x2).pow(2) + (y1 - y2).pow(2);
        let rpow = (r1 + r2).pow(2);

        // 円が完全に離れている
        if dpow > rpow {
            println!("No");
            continue;
        }

        // 円2が完全に中に入っている
        if dpow < (r1 - r2).pow(2) {
            println!("No");
            continue;
        }

        println!("Yes");
    }
}
