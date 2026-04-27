#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

/*
# 条件を満たさないものをカウントする

*/

fn two_char(s1: char, s2: char, S: &Vec<char>) -> i64 {
    let mut cnt: HashMap<i64, i64> = HashMap::new();
    cnt.insert(0, 1);
    let mut c = 0;
    for &s in S.iter() {
        if s == s1 {
            c += 1;
        } else if s == s2 {
            c -= 1;
        }
        *cnt.entry(c).or_default() += 1;
    }

    let mut ans = 0;
    for (_, &v) in cnt.iter() {
        ans += v * (v - 1) / 2;
    }
    ans
}

fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let mut ans = ((N + 1) * N / 2) as i64;
    ans -= two_char('A', 'B', &S);
    ans -= two_char('A', 'C', &S);
    ans -= two_char('C', 'B', &S);

    // 引き過ぎている分を戻す
    let mut cnt: HashMap<(i64, i64), i64> = HashMap::new();
    cnt.insert((0, 0), 1);
    let mut cnt_ab = 0;
    let mut cnt_bc = 0;
    for &s in S.iter() {
        if s == 'A' {
            cnt_ab += 1;
        } else if s == 'B' {
            cnt_ab -= 1;
            cnt_bc += 1;
        } else {
            cnt_bc -= 1;
        }
        *cnt.entry((cnt_ab, cnt_bc)).or_default() += 1;
    }

    for (_, &v) in cnt.iter() {
        ans += v * (v - 1);
    }

    println!("{ans}");
}
