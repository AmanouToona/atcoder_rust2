#![allow(non_snake_case)]
use ac_library::LazySegtree;
use ac_library::ModInt998244353 as mint;
use ac_library::{MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;
struct M;
impl Monoid for M {
    type S = (mint, mint);
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
    fn identity() -> Self::S {
        (mint::new(0), mint::new(1))
    }
}

struct F;
impl MapMonoid for F {
    type F = Option<mint>;
    type M = M;

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if let Some(v) = f {
            (v * x.1, x.1)
        } else {
            *x
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if f.is_some() {
            *f
        } else {
            *g
        }
    }
    fn identity_map() -> Self::F {
        None
    }
}

fn main() {
    input! {
        (n, m): (usize, usize),
        A: [usize; n],
        LR: [(usize, usize); m],
    }

    let mut seg = LazySegtree::<F>::new(n);
    for (i, &a) in A.iter().enumerate() {
        seg.set(i, (mint::new(a), mint::new(1)));
    }

    for &(l, r) in LR.iter() {
        let l = l - 1;
        let r = r - 1;

        let sum = seg.prod(l..=r);
        seg.apply_range(l..=r, Some(sum.0 / mint::new(r - l + 1)));
    }

    let mut ans = Vec::new();
    for i in 0..n {
        ans.push(seg.get(i).0);
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}")
}
