use proconio::input;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

fn calc_delta(set: &BTreeSet<i64>, t: i64, x: i64) -> i64 {
    let left = set
        .range((Unbounded, Excluded(t)))
        .next_back()
        .cloned()
        .unwrap_or(i64::MIN);
    let right = set
        .range((Excluded(t), Unbounded))
        .next()
        .cloned()
        .unwrap_or(i64::MAX);

    let di = t
        .saturating_sub(left)
        .abs()
        .min(right.saturating_sub(t).abs());

    if di < (x - t).abs() {
        return 0;
    }

    di - (x - t).abs()
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: [i64; N],
    }

    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(X[0]);

    let mut ans = X[0] * 2;
    println!("{}", ans);

    for x in X.iter().skip(1) {
        let left = set.range((Unbounded, Excluded(x))).next_back().unwrap();
        let right = set.range((Excluded(x), Unbounded)).next();

        match right {
            Some(right) => {
                ans += (x - left).min(right - x);
            }
            None => {
                ans += x - left;
            }
        }

        ans -= calc_delta(&set, *left, *x);

        if let Some(right) = right {
            ans -= calc_delta(&set, *right, *x);
            eprintln!("{}", ans);
        }

        set.insert(*x);
        println!("{}", ans);
    }
}
