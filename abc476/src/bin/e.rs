#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (n, m): (usize, usize),
        P: [usize; n],
        LR: [(usize, usize); m],
    }

    struct M;
    impl Monoid for M {
        type S = (usize, usize);
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            if a.0 < b.0 {
                *a
            } else {
                *b
            }
        }
        fn identity() -> Self::S {
            (usize::MAX, 0)
        }
    }

    let mut min_tree = Segtree::<M>::new(n);

    struct N;
    impl Monoid for N {
        type S = (usize, usize);
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            if a.0 < b.0 {
                *b
            } else {
                *a
            }
        }
        fn identity() -> Self::S {
            (0, 0)
        }
    }
    let mut max_tree = Segtree::<N>::new(n);
    for (i, &p) in P.iter().enumerate() {
        min_tree.set(i, (p, i));
        max_tree.set(i, (p, i));
    }

    for &(l, r) in LR.iter() {
        let max = max_tree.prod(l - 1..r);
        let min = min_tree.prod(l - 1..r);

        max_tree.set(min.1, (max.0, min.1));
        max_tree.set(max.1, (min.0, max.1));
        min_tree.set(min.1, (max.0, min.1));
        min_tree.set(max.1, (min.0, max.1));
    }

    let mut ans = Vec::new();
    for i in 0..n {
        ans.push(min_tree.get(i).0);
    }
    println!("{}", ans.iter().join(" "));
}
