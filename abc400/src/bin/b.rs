#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut x = 0;
    for i in 0..=M {
        x += N.pow(i as u32);
        if x > 10usize.pow(9u32) {
            println!("inf");
            return;
        }
    }

    println!("{x}");
}
