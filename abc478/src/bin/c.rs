#![allow(non_snake_case)]
use std::{collections::BTreeMap, usize};

use either::Either::Right;
use proconio::input;
fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }

    let mut res: BTreeMap<usize, usize> = BTreeMap::new();
    for &a in A.iter() {
        *res.entry(a).or_default() += 1;
    }

    let mut right: BTreeMap<usize, usize> = BTreeMap::new();
    let mut can_right = 0;
    for (i, &a) in A.iter().rev().enumerate() {
        *res.entry(a).or_default() -= 1;
        if res[&a] == 0 {
            res.remove(&a);
        }
        *right.entry(a).or_default() += 1;

        let right_min = right.keys().next().unwrap();
        let res_max = res.keys().last().unwrap_or(&usize::MAX);

        // eprintln!("{i} {right_min} {res_max} ");

        if right_min >= res_max {
            can_right = i + 1;
        } else {
            break;
        }
    }

    if can_right + K >= N {
        println!("Yes");
        return;
    }

    let must_left = N - can_right - K;
    let mut max = 0;
    for &a in A.iter().take(must_left) {
        max = max.max(a);
    }

    eprintln!("max: {max}, can_right: {can_right}, must_left: {must_left}");
    for &a in A.iter().take(N - can_right).skip(must_left) {
        if a < max {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
