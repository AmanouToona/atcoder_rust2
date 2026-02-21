#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (M, A, B): (usize, usize, usize),
    }

    let mut ok = HashSet::new();
    let mut ng = HashSet::new();
    let mut ans = 0;

    for (x, y) in iproduct!(0..M, 0..M) {
        let mut search = HashSet::new();
        let mut s1 = x;
        let mut s2 = y;

        loop {
            // ガード
            if search.contains(&(s1, s2)) {
                ok.insert((s1, s2));
            }
            if s1 % M == 0 || s2 % M == 0 {
                ng.insert((s1, s2));
            }

            if ok.contains(&(s1, s2)) {
                for s in search.iter() {
                    ok.insert(*s);
                }
                ans += 1;
                break;
            }
            if ng.contains(&(s1, s2)) {
                for s in search.iter() {
                    ng.insert(*s);
                }
                break;
            }

            search.insert((s1, s2));
            // 更新
            let new = (A * s1 + B * s2) % M;
            s2 = s1;
            s1 = new;
        }
    }

    println!("{ans}");
}
