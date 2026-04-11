#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        N: usize,
        lr: [(usize, usize); N],
    }
    // eprintln!("");

    /*
    全ての可能性を考える 愚直だと 2 ** N になる。
    a, b のチームのどちらかを考えて最後に2倍すれば一般性を失わない.
    dp らしさを感じる
    l, r の制約の確認を逐次行うことが難しい。　最後にできる人数？, 最もきつい制約を持っておけばよい？
    l でソートして?
     */
    let mut ls: Vec<Vec<usize>> = vec![Vec::new(); N + 1];
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

    let mut a = HashSet::new();
    let mut b = HashSet::new();
    let mut both = HashSet::new();
    let mut ans = mint::new(0);

    for (i, j) in (1..N).zip((1..N).rev()) {
        for l in ls[i].iter() {
            a.insert(*l);
            if b.contains(l) {
                both.insert(*l);
            }
        }
        for r in rs[i - 1].iter() {
            a.remove(r);
            both.remove(r);
        }

        for r in rs[j].iter() {
            b.insert(*r);
            if a.contains(r) {
                both.insert(*r);
            }
        }
        for l in ls[j + 1].iter() {
            b.remove(l);
            both.remove(l);
        }
        // println!("{i} {j}");
        // println!("{:?}, {:?}, {:?}", a, b, both);

        let static_a = a.len() - both.len();
        if static_a > i {
            continue;
        }
        let to_have = i - static_a;
        if to_have > both.len() {
            continue;
        }
        ans += frac[both.len()] * finv[to_have] * finv[both.len() - to_have];
    }
    println!("{ans}");
}
