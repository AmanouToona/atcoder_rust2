#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (M, D): (usize, usize)
    }

    let is = match (M, D) {
        (1, 7) => true,
        (3, 3) => true,
        (5, 5) => true,
        (7, 7) => true,
        (9, 9) => true,
        _ => false,
    };

    if is {
        println!("Yes");
    } else {
        println!("No");
    }
}
