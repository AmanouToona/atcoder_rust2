#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashSet;
/*
対角2点が決まれば残りの点の位置も決定される。
N ** 2 で計算できる
*/
fn main() {
    input! {
        N: usize,
        mut xy: [(i64, i64); N],
    }
    xy.sort();
    let set: HashSet<(i64, i64)> = HashSet::from_iter(xy.iter().cloned());
    let mut ans = 0;
    for i in 0..N {
        for j in i + 1..N {
            let n1 = xy[i];
            let n2 = xy[j];

            if n1.1 >= n2.1 {
                continue;
            }
            if n1.0 == n2.0 {
                continue;
            }

            if set.contains(&(n1.0, n2.1)) && set.contains(&(n2.0, n1.1)) {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
