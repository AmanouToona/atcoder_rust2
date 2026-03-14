#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main() {
    input! {
        (N, L, R): (usize, usize, usize),
        S: Chars,
    }

    let mut cnt: HashMap<char, usize> = HashMap::new();
    for i in L..=R {
        *cnt.entry(S[i]).or_default() += 1;
    }

    let mut ans = 0;
    for i in 0..N {
        ans += cnt.get(&S[i]).unwrap_or(&0);

        if i + L < N {
            *cnt.entry(S[i + L]).or_default() -= 1;
        }
        if i + R + 1 < N {
            *cnt.entry(S[i + R + 1]).or_default() += 1;
        }
    }

    println!("{ans}");
}
