#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;
fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            (N, C): (usize, usize),
            S: [Chars; N]
        }

        let mut wall_start = vec![0; N];
        for (r, s) in S.iter().enumerate() {
            for (c, s) in s.iter().enumerate() {
                if *s == '#' {
                    wall_start[c] = r;
                }
            }
        }

        let mut pre_pos = BTreeSet::new();
        pre_pos.insert(C - 1);
        for (i, s) in S.iter().enumerate().rev().skip(1) {
            let mut nxt_pos = BTreeSet::new();
            for &u in pre_pos.iter() {
                for d in [!0, 0, 1].iter() {
                    let v = u.wrapping_add(*d);
                    if v >= N {
                        continue;
                    }

                    if s[v] == '#' && wall_start[v] > i {
                        continue;
                    }

                    if wall_start[v] <= i {
                        wall_start[v] = 0;
                    }

                    nxt_pos.insert(v);
                }
            }
            pre_pos = nxt_pos;
        }

        let ans: String = (0..N)
            .map(|x| if pre_pos.contains(&x) { 1 } else { 0 })
            .join("");
        println!("{ans}");
    }
}
