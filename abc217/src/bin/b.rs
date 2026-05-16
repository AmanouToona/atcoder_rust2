#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: [Chars; 3]
    }

    let contests = ['B', 'R', 'G', 'H'];
    let s: Vec<char> = s.iter().map(|x| x[1]).collect();
    let c = contests.iter().find(|&&c| !s.contains(&c)).unwrap();

    println!("A{}C", c);
}
