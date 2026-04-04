#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {X: f64}
    if X >= 38. {
        println!("1")
    } else if X >= 37.5 {
        println!("2")
    } else {
        println!("3")
    }
}
