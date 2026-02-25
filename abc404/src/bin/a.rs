#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    for i in 0..26 {
        let i: char = (b'a' + i) as char;
        if S.contains(&i) {
            continue;
        } else {
            println!("{i}");
            return;
        }
    }
}
