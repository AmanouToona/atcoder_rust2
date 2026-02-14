#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
fn main() {
    input! {
        (N, Q): (usize, usize),
        mut S: Chars,
    }

    struct M;
    impl Monoid for M {
        type S = usize;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(&b)
        }
        fn identity() -> Self::S {
            0
        }
    }

    let mut l2r: BTreeMap<usize, usize> = BTreeMap::new();
    let mut length = Segtree::<M>::new(N + 1);

    let mut left = 0;
    for (i, &s) in S.iter().chain(['.'].iter()).enumerate() {
        if s != S[left] {
            l2r.insert(left, i);
            left = i;
            length.set(left, i - left);
        }
    }

    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                input! {(i, x): (usize, char)}
                let i = i - 1;

                // 左の処理
                if let Some((&idx, &v)) = l2r.range((Unbounded, Excluded(&i))).next_back() {
                    // 切断
                    if S[idx] == S[i] {
                        if let Some(u) = l2r.get_mut(&idx) {
                            *u = i;
                            length.set(idx, i - idx);
                        };
                        l2r.insert(i, i + 1);
                        length.set(i, 1);
                    }
                    // 結合
                    else if S[idx] == x {
                        if let Some(u) = l2r.get_mut(&idx) {
                            *u = i + 1;
                            length.set(idx, i + 1 - idx);
                        }
                        l2r.remove(&i);
                        length.set(i, 0);
                    }
                }

                // 右の処理
                if let Some((&idx, &v)) = l2r.range((Included(&i), Unbounded)).next() {
                    //　切断
                    if idx == i {
                        l2r.remove(&idx);
                        l2r.insert(idx, i + 1);
                        length.set(idx, i + 1 - idx);
                        l2r.insert(idx + 1, v);
                        length.set(idx + 1, v - (idx + 1));
                    }
                    // 結合
                    else if x == S[idx] {
                        let (&l, _) = l2r.range((Unbounded, Included(&i))).next_back().unwrap();
                        l2r.remove(&idx);
                        length.set(idx, 0);
                        l2r.remove(&l);
                        l2r.insert(l, v);
                        length.set(l, v - l);
                    }
                }
            }
            2 => {
                input! {(l, r): (usize, usize)}
                let l = l - 1;
                let r = r - 1;
                let mut ans = 1;
                // 左の処理
                if let Some((&ll, &lr)) = l2r.range(..l).next_back() {
                    ans = ans.max(lr.max(l) - l);
                }
                // 右の処理
                if let Some((&lr, &rr)) = l2r.range(l..=r).next_back() {
                    ans = ans.max((r + 1).wrapping_sub(lr));
                }
                // 全域の処理
                let (&l, _) = l2r.range(l..).next().unwrap();
                let (&r, _) = l2r.range(..r).next_back().unwrap();
                ans = ans.max(length.prod(l..r));

                println!("{ans}");
            }
            _ => {
                panic!("wrong query")
            }
        }
    }
}
