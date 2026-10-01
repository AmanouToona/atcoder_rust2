#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        N: usize,
        D: u64,
        mut X: [i64; N],
    }

    X.push(-3_000_000_000);
    X.push(3_000_000_000);

    let mut X: Vec<(usize, i64)> = X.into_iter().enumerate().collect();
    X.sort_by_key(|x| x.1);

    let mut ans = Vec::new();
    for (&pre, &curr, &nxt) in X.iter().tuple_windows::<(_, _, _)>() {
        if curr.1.abs_diff(pre.1) >= D && curr.1.abs_diff(nxt.1) >= D {
            ans.push(curr.0 + 1);
        }
    }

    ans.sort();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
