#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {S: usize}

    if (200..=299).contains(&S) {
        println!("Success")
    } else {
        println!("Failure")
    }
}
