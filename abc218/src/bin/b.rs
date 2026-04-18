#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        P: [usize; 26],
    }

    let ans: String = P.iter().map(|x| (*x as u8 - 1 + b'a') as char).join("");
    println!("{ans}");
}
