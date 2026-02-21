#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let T: HashSet<&char> = HashSet::from_iter(T.iter());

    for i in 1..S.len() {
        if S[i].is_uppercase() && !T.contains(&S[i - 1]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
