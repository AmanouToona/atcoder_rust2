#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        X: usize
    }

    if (3..=18).contains(&X) {
        println!("Yes");
    } else {
        println!("No");
    }
}
