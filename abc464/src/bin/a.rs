#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut count: i32 = 0;

    for &c in S.iter() {
        if c == 'E' {
            count += 1;
        } else {
            count -= 1;
        }
    }

    if count > 0 {
        println!("East");
    } else {
        println!("West");
    }
}
