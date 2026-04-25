#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        (N, K): (usize , usize),
        A: [usize; N]
    }

    let mut cnt: HashMap<usize, usize> = HashMap::new();
    for &a in A.iter() {
        *cnt.entry(a).or_default() += 1;
    }

    let mut summed = Vec::new();
    for (k, v) in cnt.iter() {
        summed.push(*k * *v);
    }
    summed.sort_by(|x, y| y.cmp(&x));

    let ans: usize = summed.iter().skip(K).sum();

    println!("{ans}");
}
