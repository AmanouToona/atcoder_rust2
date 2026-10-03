#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        S: String,
    }

    if S.chars().last().unwrap() != 'e' {
        println!("{}er", S.to_string());
    } else {
        println!("{}r", S.to_string());
    }
}
