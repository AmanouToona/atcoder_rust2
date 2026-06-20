#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (N, X): (usize, char),
        S: [Chars; N]
    }

    let n = (X as u8 - b'A') as usize;

    for s in S.iter() {
        if s[n] == 'o' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
