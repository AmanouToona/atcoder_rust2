#![allow(non_snake_case)]
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
    let mut s = Vec::new();
    for c in S.iter() {
        if !s_pos.contains_key(c) {
            s_pos.insert(*c, s_pos.len());
        }
        s.push(s_pos[c]);
    }

    let primes = eratosthenes(100_000_000);
    for prime in primes {
        let mut tmp = prime;
        let mut digits = Vec::new();
        while tmp != 0 {
            digits.push(tmp % 10);
            tmp /= 10;
        }

        let digits: Vec<usize> = digits.into_iter().rev().collect();

        if digits.len() != s.len() {
            continue;
        }

        let mut p_pos = [usize::MAX; 10];
        let mut nxt_id = 0;
        for &digit in digits.iter() {
            if p_pos[digit] == usize::MAX {
                p_pos[digit] = nxt_id;
                nxt_id += 1;
            }
        }

        let mut p = Vec::new();
        for &digit in digits.iter() {
            p.push(p_pos[digit]);
        }

        if s.len() != p.len() {
            continue;
        }

        if s == p {
            println!("{}", prime);
            return;
        }
    }

    println!("-1")
}
