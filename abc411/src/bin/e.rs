#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;
use std::collections::BTreeSet;
fn main() {
    input! {
        N: usize,
        A: [[usize; 6]; N],
    }

    struct M;
    impl Monoid for M {
        type S = mint;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a * b
        }
        fn identity() -> Self::S {
            mint::new(1)
        }
    }

    // multiset の代替  (目の大きさ, サイコロの番号, fuller)
    let mut compression = Vec::new();
    let mut set = BTreeSet::new();
    for (i, a) in A.iter().enumerate() {
        for &a in a.iter() {
            set.insert((a, i, set.len()));
            compression.push(a);
        }
    }

    compression.sort();
    compression.dedup();

    let mut seg = Segtree::<M>::new(N);
    for i in 0..N {
        seg.set(i, mint::new(0));
    }

    let mut pre = mint::new(0);
    let mut ans = mint::new(0);
    for &c in compression.iter() {
        while let Some(&(face, dice_n, _)) = set.first() {
            if face <= c {
                set.pop_first();
                let p = seg.get(dice_n);
                seg.set(dice_n, p + mint::new(1) / mint::new(6));
            } else {
                break;
            }
        }
        let cnt = seg.prod(0..N);
        ans += mint::new(c) * (cnt - pre);
        pre = cnt;
    }
    println!("{ans}");
}
