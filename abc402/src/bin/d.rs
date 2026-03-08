#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        (N, M): (usize, usize),
        mut AB: [(usize, usize); M],
    }

    // 交わらないのは平行のもののみ
    let mut ans = 0;
    let mut set = HashMap::new();
    AB = AB.iter().map(|&x| ((x.0 - 1) * 2, (x.1 - 1) * 2)).collect();

    for (seen, &(a, b)) in AB.iter().enumerate() {
        let mut mid = (a + b) / 2;
        if mid <= N {
            mid += N;
        }

        if let Some(x) = set.get(&mid) {
            ans += seen - x;
        } else {
            ans += seen;
        }

        *set.entry(mid).or_default() += 1;
    }

    println!("{ans}");
}
