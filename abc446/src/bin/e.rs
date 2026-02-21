#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (M, A, B): (usize, usize, usize),
    }

    let mut ok = HashSet::new();
    let mut ng = HashSet::new();
    let mut ans = 0;

    for x in 0..M {
        for y in 0..M {
            let mut search = HashSet::new();
            let mut s1 = x;
            let mut s2 = y;

            loop {
                // eprintln!("{s1} {s2} {}", search.len());
                if ok.contains(&(s1, s2)) {
                    for &s in search.iter() {
                        ok.insert(s);
                    }
                    ans += 1;
                    break;
                }
                if ng.contains(&(s1, s2)) {
                    for &s in search.iter() {
                        ng.insert(s);
                    }
                    break;
                }

                search.insert((s1, s2));
                if s1 % M == 0 || s2 % M == 0 {
                    for &s in search.iter() {
                        ng.insert(s);
                    }
                    break;
                }
                let new = (A * s1 + B * s2) % M;
                s2 = s1 % M;
                s1 = new % M;
                if search.contains(&(s1, s2)) {
                    for &s in search.iter() {
                        ok.insert(s);
                    }
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{ans}");
}
