#![allow(non_snake_case)]
use ac_library::LazySegtree;
use ac_library::MapMonoid;
use ac_library::ModInt998244353 as Mint;
use ac_library::Monoid;
use itertools::Itertools;
use proconio::input;

#[derive(Clone, Copy)]
struct S {
    val: Mint,
    len: usize,
}

struct M;
impl Monoid for M {
    type S = S;
    fn identity() -> Self::S {
        S {
            val: Mint::new(0),
            len: 0,
        }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        S {
            val: a.val + b.val,
            len: a.len + b.len,
        }
    }
}

struct F;
impl MapMonoid for F {
    type M = M;
    type F = Option<Mint>;
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
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if let Some(v) = f {
            S {
                val: *v * x.len,
                len: x.len,
            }
        } else {
            *x
        }
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
        seg.set(
            i,
            S {
                val: Mint::new(a),
                len: 1,
            },
        );
    }

    for &(l, r) in LR.iter() {
        let l = l - 1;
        let r = r - 1;

        let sum = seg.prod(l..=r);
        let ave = sum.val / Mint::new(r - l + 1);

        seg.apply_range(l..=r, Some(ave));
    }

    let mut ans = Vec::new();
    for i in 0..n {
        ans.push(seg.get(i).val);
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
