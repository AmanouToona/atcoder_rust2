#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Ordering;
use std::collections::HashMap;
fn main() {
    input! {
        X: Chars,
        N: usize,
       mut  S: [Chars; N],
    }

    let mut alp2num: HashMap<&char, usize> = HashMap::new();
    for x in X.iter() {
        alp2num.insert(x, alp2num.len());
    }

    let cmp = |x: &Vec<char>, y: &Vec<char>| -> Ordering {
        for (sx, sy) in x.iter().zip(y.iter()) {
            if alp2num[sx] != alp2num[sy] {
                return alp2num[sx].cmp(&alp2num[sy]);
            }
        }

        if x.len() != y.len() {
            return x.len().cmp(&y.len());
        }

        Ordering::Equal
    };

    S.sort_by(|x, y| cmp(x, y));

    for i in S.iter() {
        let ans = i.iter().join("");
        println!("{ans}");
    }
}
