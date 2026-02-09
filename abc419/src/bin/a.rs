#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main() {
    input! {
        S: Chars
    }

    let mut converter = HashMap::new();
    converter.insert("red", "SSS");
    converter.insert("blue", "FFF");
    converter.insert("green", "MMM");

    let S: String = S.iter().collect();
    let ans = converter.get(&S.as_str()).unwrap_or(&"Unknown");

    println!("{ans}");
}
