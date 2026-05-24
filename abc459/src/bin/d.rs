#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main() {
    input! {
        T: usize,
    }

    'outer: for _ in 0..T {
        input! {
            mut S: Chars,
        }

        let mut cnt: HashMap<char, usize> = HashMap::new();
        for &s in S.iter() {
            *cnt.entry(s).or_default() += 1;
        }

        S.sort_by(|i, j| cnt.get(j).cmp(&cnt.get(i)).then((*i).cmp(j)));

        let mut ans = vec!['.'; S.len()];
        let mut i = 0;
        let mut j = 0;
        while i < ans.len() {
            ans[i] = S[j];
            i += 2;
            j += 1;
        }
        i = 1;
        while i < ans.len() {
            ans[i] = S[j];
            i += 2;
            j += 1;
        }

        for (i, j) in ans.iter().zip(ans.iter().skip(1)) {
            if i == j {
                println!("No");
                continue 'outer;
            }
        }

        let ans: String = ans.iter().join("");
        println!("Yes");
        println!("{ans}");
    }
}
