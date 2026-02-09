#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        (X, Y): (String, String)
    }

    let mut to_version = HashMap::new();
    to_version.insert("Ocelot", 1);
    to_version.insert("Serval", 2);
    to_version.insert("Lynx", 3);

    if to_version.get(&X.as_str()).unwrap() >= to_version.get(&Y.as_str()).unwrap() {
        println!("Yes");
    } else {
        println!("No");
    }
}
