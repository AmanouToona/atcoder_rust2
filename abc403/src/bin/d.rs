#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        (N, D): (usize, usize),
        A: [usize; N],
    }

    let mut A: Vec<usize> = A.iter().map(|x| *x + 1).collect();
    A.sort();

    let max = *A.iter().max().unwrap();
    // dp[number][use]
    let mut dp = vec![vec![0; 2]; max + 1];
    let mut cnt: HashMap<usize, usize> = HashMap::new();

    for &a in A.iter() {
        *cnt.entry(a).or_default() += 1;
    }
    if D == 0 {
        println!("{}", N - cnt.len());
        return;
    }

    for v in 1..=max {
        let u = v.saturating_sub(D);

        let &can_use = cnt.get(&v).unwrap_or(&0);

        // use number v
        if can_use != 0 {
            dp[v][0] = dp[u][1] + can_use;
        }

        // unuse number v
        dp[v][1] = *dp[u].iter().max().unwrap_or(&0);
    }

    let mut can_use = 0;
    for i in (1..=max).rev().take(D) {
        can_use += dp[i].iter().max().unwrap();
    }

    let ans = N - can_use;
    println!("{ans}");
}
