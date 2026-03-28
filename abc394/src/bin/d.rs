#![allow(non_snake_case)]
use amplify::confinement::Collection;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
fn main() {
    input! {
        S: Chars,
    }

    let mut q = VecDeque::new();
    for &s in S.iter() {
        if s == '(' || s == '[' || s == '<' {
            q.push(s);
        } else {
            if let Some(t) = q.pop_back() {
                if (s == ')' && t == '(') || (s == ']' && t == '[') || (s == '>' && t == '<') {
                    continue;
                }
                println!("No");
                return;
            }
            println!("No");
            return;
        }
    }
    if !q.is_empty() {
        println!("No");
        return;
    }

    println!("Yes");
}
