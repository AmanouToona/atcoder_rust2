#![allow(non_snake_case)]
use num::Integer;
use proconio::input;
fn main() {
    input! {
        (_, K): (usize, usize),
        A: [usize; K],
    }

    let M = K / 2;
    let mut cost_left = Vec::new();
    for i in 0..M {
        cost_left.push(A[i * 2 + 1] - A[i * 2]);
    }
    let mut pref_left = vec![0; M + 1];
    for i in 0..M {
        pref_left[i + 1] = pref_left[i] + cost_left[i];
    }

    if A.len().is_even() {
        println!("{}", pref_left.last().unwrap());
        return;
    }

    let mut cost_right = Vec::new();
    for i in 0..M {
        cost_right.push(A[i * 2 + 2] - A[i * 2 + 1]);
    }

    let mut suff_right = vec![0; M + 1];
    for i in (0..M).rev() {
        suff_right[i] = suff_right[i + 1] + cost_right[i];
    }

    let ans: usize = pref_left
        .iter()
        .zip(suff_right.iter())
        .map(|x| x.0 + x.1)
        .min()
        .unwrap();
    println!("{ans}");
}
