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
        type S = (usize, usize, usize, usize);
        fn identity() -> Self::S {
            (usize::MAX, 0, 0, 0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            let min: (usize, usize) = if a.0 < b.0 { (a.0, a.1) } else { (b.0, b.1) };
            let max: (usize, usize) = if a.2 > b.2 { (a.2, a.3) } else { (b.2, b.3) };

            (min.0, min.1, max.0, max.1)
        }
    }

    let initial_leaves: Vec<_> = P.iter().enumerate().map(|(i, &p)| (p, i, p, i)).collect();
    let mut tree = Segtree::<M>::from(initial_leaves);

    for &(l, r) in LR.iter() {
        let node = tree.prod(l - 1..r);

        tree.set(node.1, (node.2, node.1, node.2, node.1)); // 最小を最大に
        tree.set(node.3, (node.0, node.3, node.0, node.3)); // 最大を最小に
    }

    let ans = (0..n).map(|i| tree.get(i).0.to_string()).join(" ");

    println!("{}", ans);
}
