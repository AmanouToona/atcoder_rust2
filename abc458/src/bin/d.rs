#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;
use std::collections::BTreeSet;
use std::collections::HashMap;
/*
常に中央値を算出する

a, b, x の値は大きいが、座標圧縮で高々 2 * 10 **5 までは押さえ込める
セグ木 & 2分探索で殴ることはできそう より賢い実装はあるか？

*/

fn main() {
    input! {
        X: usize,
        Q: usize,
        ab: [(usize, usize); Q],
    }

    let mut set = BTreeSet::new();
    set.insert(X);
    for &(a, b) in ab.iter() {
        set.insert(a);
        set.insert(b);
    }

    let mut num_id = HashMap::new();
    for (i, n) in set.iter().enumerate() {
        num_id.insert(*n, i);
    }

    let mut id_num = HashMap::new();
    for (&k, &v) in num_id.iter() {
        id_num.insert(v, k);
    }

    struct M;
    impl Monoid for M {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    let mut seg = Segtree::<M>::new(num_id.len());
    seg.set(num_id[&X], seg.get(num_id[&X]) + 1);

    for (i, &(a, b)) in ab.iter().enumerate() {
        seg.set(num_id[&a], seg.get(num_id[&a]) + 1);
        seg.set(num_id[&b], seg.get(num_id[&b]) + 1);

        if seg.get(0) > (i + 1) {
            println!("{}", id_num[&0]);
            continue;
        }

        let mut ng = 0;
        let mut ok = num_id.len() - 1;

        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if seg.prod(0..=mid) > i + 1 {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{}", id_num[&ok]);
    }
}
