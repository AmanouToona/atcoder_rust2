#![allow(non_snake_case)]
use itertools::Itertools;
use num::Integer;

use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (N, K): (usize, usize),
        S: Chars,
    }

    let mut T = S.clone();
    for i in 0..N {
        if T[i] != '?' {
            continue;
        }
        if i > 0 && T[i - 1] == 'o' {
            T[i] = '.';
            continue;
        }
        if i + 1 < N && T[i + 1] == 'o' {
            T[i] = '.';
        }
    }

    let mut rle = Vec::new();
    rle.push((T[0], 1));
    for &t in T.iter().skip(1) {
        let last = rle.last_mut().unwrap();
        if t != last.0 {
            rle.push((t, 1));
        } else {
            last.1 += 1;
        }
    }

    let max: usize = rle
        .iter()
        .filter(|&x| x.0 == '?')
        .map(|&x| x.1.div_ceil(&2))
        .sum();

    let now: usize = rle.iter().filter(|&x| x.0 == 'o').map(|&x| x.1).sum();

    if now == K {
        for t in T.iter_mut() {
            if *t == '?' {
                *t = '.';
            }
        }
        let ans: String = T.iter().join("");
        println!("{ans}");
        return;
    }

    if max + now > K {
        let ans: String = T.iter().join("");
        println!("{ans}");
        return;
    }

    let mut i = 0;
    for &(t, len) in rle.iter() {
        if t == '?' && len.is_odd() {
            for j in 0..len {
                if j.is_even() {
                    T[i + j] = 'o';
                } else {
                    T[i + j] = '.';
                }
            }
        }
        i += len;
    }

    let ans: String = T.iter().join("");
    println!("{ans}");
}
