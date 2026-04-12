#![allow(non_snake_case)]
use ac_library::LazySegtree;
use ac_library::{MapMonoid, Monoid};
use proconio::input;
fn main() {
    input! {
        (H, W, N): (usize, usize, usize),
        rcl: [(usize, usize, usize); N],
    }

    let mut rcli: Vec<(usize, usize, usize, usize)> = rcl
        .iter()
        .enumerate()
        .map(|(i, x)| (x.0, x.1, x.2, i))
        .collect();

    rcli.sort_by(|x, y| y.0.cmp(&x.0));

    struct M;
    impl Monoid for M {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(&b)
        }
    }

    struct F;
    impl MapMonoid for F {
        type M = M;
        type F = usize;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            *f.max(g)
        }
        fn identity_map() -> Self::F {
            0
        }
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            *x.max(f)
        }
    }

    let mut ans = vec![0; N];
    let mut seg = LazySegtree::<F>::new(W);
    for &(_, c, l, i) in rcli.iter() {
        let c = c - 1;
        let h = seg.prod(c..c + l);
        ans[i] = H - h;
        seg.apply_range(c..c + l, h + 1);
    }

    for i in ans.iter() {
        println!("{i}");
    }
}
