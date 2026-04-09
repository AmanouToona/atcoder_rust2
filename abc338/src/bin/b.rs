#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut count = [0; 26];
    for &s in S.iter() {
        count[(s as u8 - b'a') as usize] += 1;
    }

    let max = count.iter().max().unwrap();
    for (i, c) in count.iter().enumerate() {
        if c == max {
            println!("{}", (i as u8 + b'a') as char);
            return;
        }
    }
}
