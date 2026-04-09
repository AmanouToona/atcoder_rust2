#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N],
        M: usize,
        S: [Chars; M],
    }

    // 長さ, ポジション, アルファベット
    let mut set = HashSet::new();
    for s in S.iter() {
        for (i, ss) in s.iter().enumerate() {
            set.insert((s.len(), i, *ss));
        }
    }

    'outer: for s in S.iter() {
        if s.len() != N {
            println!("No");
            continue;
        }
        for (i, &(a, b)) in AB.iter().enumerate() {
            if !set.contains(&(a, b - 1, s[i])) {
                println!("No");
                continue 'outer;
            }
        }
        println!("Yes");
    }
}
