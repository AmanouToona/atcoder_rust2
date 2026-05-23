#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
/*
最も個数の多い文字が、他の文字の総数 + 1 より多いと不可能
*/
fn main() {
    input! {T: usize}

    'outer: for _ in 0..T {
        input! { mut  S: Chars}

        let mut set: HashMap<char, usize> = HashMap::new();
        for s in S.iter() {
            *set.entry(*s).or_default() += 1;
        }

        S.sort_by_key(|x| -(*set.get(x).unwrap() as i64));
        let mut ans = vec!['.'; S.len() + 1];

        let max = *set.values().max().unwrap();
        if max > S.len().div_ceil(2) {
            println!("No");
            continue 'outer;
        }

        for (i, &s) in S.iter().enumerate() {}

        println!("Yes");
        let ans: String = ans.iter().skip(1).join("");
        println!("{ans}");
    }
}
