#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main() {
    input! {
        N: usize,
        st: [(Chars, Chars); N]
    }

    let mut set: HashSet<(String, String)> = HashSet::new();
    for (s, t) in st.iter() {
        let s = s.iter().join("");
        let t = t.iter().join("");
        if set.contains(&(s.clone(), t.clone())) {
            println!("Yes");
            return;
        } else {
            set.insert((s, t));
        }
    }
    println!("No");
}
