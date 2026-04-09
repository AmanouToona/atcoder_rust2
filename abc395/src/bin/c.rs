#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn judge(A: &[usize], len: usize) -> bool {
    let mut cnt: HashMap<usize, usize> = HashMap::new();
    let mut dup: HashSet<usize> = HashSet::new();

    for &a in A.iter().take(len - 1) {
        *cnt.entry(a).or_default() += 1;
        if cnt.get(&a).unwrap_or(&0) >= &2 {
            dup.insert(a);
        }
    }

    for (i, &a) in A.iter().take(A.len() - len + 1).enumerate() {
        let right = i + len - 1;
        *cnt.entry(A[right]).or_default() += 1;
        if cnt.get(&A[right]).unwrap_or(&0) >= &2 {
            dup.insert(A[right]);
        }

        if !dup.is_empty() {
            return true;
        }

        *cnt.entry(a).or_default() -= 1;
        if cnt.get(&a).unwrap_or(&0) < &2 {
            dup.remove(&a);
        }
    }

    false
}

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    if !judge(&A, N) {
        println!("-1");
        return;
    }

    let mut min = 0;
    let mut max = N;
    while max - min > 1 {
        let mid = (max + min) / 2;
        if judge(&A, mid) {
            max = mid;
        } else {
            min = mid;
        }
    }

    println!("{max}");
}
