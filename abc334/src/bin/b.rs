#![allow(non_snake_case)]
use num::Integer;
use proconio::input;
fn main() {
    input! {
        (A, M, L, R): (i128, i128, i128, i128),
    }

    let lk = (L - A).div_ceil(&M);
    let rk = (R - A).div_floor(&M);
    if lk > rk {
        println!("0")
    } else {
        println!("{}", rk - lk + 1);
    }
}
