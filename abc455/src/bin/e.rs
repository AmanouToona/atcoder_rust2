#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn cnt_sub(s: &[char], a: char, b: char) -> Vec<i64> {
    let mut res = vec![0];

    for &s in s.iter() {
        if s == a {
            res.push(1);
        } else if s == b {
            res.push(-1);
        } else {
            res.push(0);
        }
    }

    for i in 0..s.len() {
        res[i + 1] += res[i];
    }

    res
}

fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let a_b = cnt_sub(&S, 'A', 'B');
    let b_c = cnt_sub(&S, 'B', 'C');
    let a_c = cnt_sub(&S, 'A', 'C');

    let mut ans = N * (N + 1) / 2;

    let mut cnt: HashMap<(i64, i64), usize> = HashMap::new();
    for (&i, &j) in a_b.iter().zip(b_c.iter()) {
        ans += 2 * (*cnt.get(&(i, j)).unwrap_or(&0));
        *cnt.entry((i, j)).or_default() += 1;
    }
    let mut cnt: HashMap<i64, usize> = HashMap::new();
    for i in a_b.iter() {
        ans -= *cnt.get(i).unwrap_or(&0);
        *cnt.entry(*i).or_default() += 1;
    }

    let mut cnt: HashMap<i64, usize> = HashMap::new();
    for i in b_c.iter() {
        ans -= *cnt.get(i).unwrap_or(&0);
        *cnt.entry(*i).or_default() += 1;
    }

    let mut cnt: HashMap<i64, usize> = HashMap::new();
    for i in a_c.iter() {
        ans -= *cnt.get(i).unwrap_or(&0);
        *cnt.entry(*i).or_default() += 1;
    }

    println!("{ans}");
}
