#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (M, D): (usize, usize)
    }

    let is = matches!((M, D), (1, 7) | (3, 3) | (5, 5) | (7, 7) | (9, 9));
    if is {
        println!("Yes");
    } else {
        println!("No");
    }
}
