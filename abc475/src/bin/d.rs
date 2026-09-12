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
        let mut p_pos: HashMap<char, usize> = HashMap::new();
        let mut p = Vec::new();

        let prime_string = prime.to_string();
        if prime_string.len() != s.len() {
            continue;
        }
        for digit_char in prime_string.chars() {
            if !p_pos.contains_key(&digit_char) {
                p_pos.insert(digit_char, p_pos.len());
            }
            p.push(p_pos[&digit_char]);
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
