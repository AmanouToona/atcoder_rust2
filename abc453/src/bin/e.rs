#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        N: usize,
        lr: [(usize, usize); N],
    }

    /*
    全ての可能性を考える 愚直だと 2 ** N になる。
    a, b のチームのどちらかを考えて最後に2倍すれば一般性を失わない.
    dp らしさを感じる
    l, r の制約の確認を逐次行うことが難しい。　最後にできる人数？, 最もきつい制約を持っておけばよい？
    l でソートして?
     */
    let mut ls = vec![Vec::new(); N + 1];
    let mut rs = vec![Vec::new(); N + 1];
    for (i, &(l, r)) in lr.iter().enumerate() {
        ls[l].push(i);
        rs[r].push(i);
    }

    let mut frac = vec![mint::new(1); N + 1];
    let mut finv = vec![mint::new(1); N + 1];

    for i in 2..=N {
        frac[i] = frac[i - 1] * mint::new(i);
        finv[i] = finv[i - 1] / mint::new(i);
    }

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    let mut set3 = HashSet::new();

    for (i, j) in (1..N).zip(N - 1..) {
        for l in ls[i].iter() {
            set1.insert(*l);
        }

        for r in rs[j].iter() {
            set2.insert(*r);
            if set1.contains(r) {
                set3.insert(*r);
            }
        }
    }

    let mut ans = mint::new(0);
}
