#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
    }

    if (1..=125).contains(&N) {
        println!("4");
    } else if (126..=211).contains(&N) {
        println!("6");
    } else {
        println!("8")
    }
}
