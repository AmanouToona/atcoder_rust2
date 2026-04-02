#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    }

    let s: Vec<char> = "sick".to_string().chars().collect();
    let f: Vec<char> = "fine".to_string().chars().collect();

    if s1 == s && s2 == s {
        println!("1")
    } else if s1 == s && s2 == f {
        println!("2")
    } else if s1 == f && s2 == s {
        println!("3")
    } else {
        println!("4")
    }
}
