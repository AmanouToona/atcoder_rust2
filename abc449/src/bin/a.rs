#![allow(non_snake_case)]
use std::f64::consts::PI;

use proconio::input;
fn main() {
    input! {
        D: f64,
    }

    let ans = (D / 2.) * (D / 2.) * PI;
    println!("{ans}");
}
