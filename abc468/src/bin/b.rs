#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (M, D): (usize, usize),
        S: Chars,
    }

    let mut watched = vec![false; M];

    let mut near = None;
    for (i, &s) in S.iter().enumerate() {
        if s == 'G' {
            near = Some(i);
        }
        if near.is_none() {
            continue;
        }

        let j = near.unwrap();
        if i - j <= D {
            watched[i] = true;
        }
    }

    let mut near = None;
    for (i, &s) in S.iter().enumerate().rev() {
        if s == 'G' {
            near = Some(i);
        }
        if near.is_none() {
            continue;
        }

        let j = near.unwrap();
        if j - i <= D {
            watched[i] = true;
        }
    }

    let ans = watched.iter().filter(|&&x| !x).count();
    println!("{ans}");
}
