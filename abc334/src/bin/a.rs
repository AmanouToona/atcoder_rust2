#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (B, G): (usize, usize),
    }
    if B > G {
        println!("Bat");
    } else {
        println!("Glove")
    }
}
