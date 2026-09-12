#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn eratosthenes(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }

    let mut res = Vec::new();
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n {
        if is_prime[i] {
            res.push(i);
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    res
}
fn main() {
    input! {
        S: Chars,
    }

    let mut s_pos: HashMap<char, usize> = HashMap::new();
    for s in S.iter() {
        if s_pos.contains_key(s) {
            continue;
        } else {
            s_pos.insert(*s, s_pos.len());
        }
    }

    let mut s_vec: Vec<usize> = Vec::new();
    for c in S.iter() {
        s_vec.push(s_pos[c]);
    }

    let primes = eratosthenes(100_000_000); // <= 6 * 10 **6 個
    for p in primes {
        let p = p.to_string();
        if p.len() != s_vec.len() {
            continue;
        }
        let p: Vec<char> = p.chars().collect();
        let mut p_pos: HashMap<char, usize> = HashMap::new();
        for s in p.iter() {
            if p_pos.contains_key(s) {
                continue;
            } else {
                p_pos.insert(*s, p_pos.len());
            }
        }

        let mut p_vec: Vec<usize> = Vec::new();
        for c in p.iter() {
            p_vec.push(p_pos[c]);
        }

        let mut is_ans = true;
        for (p, s) in p_vec.iter().zip(s_vec.iter()) {
            if p != s {
                is_ans = false;
                break;
            }
        }

        if is_ans {
            println!("{}", p.iter().join(""));
            return;
        }
    }
    println!("-1");
}
