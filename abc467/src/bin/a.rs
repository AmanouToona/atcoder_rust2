#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (H, W): (usize, usize),
    }
    if 10000 * W >= 25 * H * H {
        println!("Yes");
    } else {
        println!("No");
    }
}
