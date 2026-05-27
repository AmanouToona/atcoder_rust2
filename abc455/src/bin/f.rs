#![allow(non_snake_case)]
use ac_library::LazySegtree;
use ac_library::MapMonoid;
use ac_library::ModInt998244353 as mint;
use ac_library::Monoid;
use proconio::input;

fn main() {
    input! {
        (N, Q): (usize, usize),
        lra: [(usize, usize, mint); Q],
    }

    // sig x
    struct M;
    impl Monoid for M {
        type S = (mint, mint);
        fn identity() -> Self::S {
            (mint::new(0), mint::new(0))
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (a.0 + b.0, a.1 + b.1)
        }
    }

    struct F;
    impl MapMonoid for F {
        type F = mint;
        type M = M;
        fn identity_map() -> Self::F {
            mint::new(0)
        }
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            (x.0 + x.1 * f, x.1)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f + g
        }
    }

    // sig x ** 2
    struct L;
    impl Monoid for L {
        type S = (mint, mint, mint); // x**2, x, n
        fn identity() -> Self::S {
            (mint::new(0), mint::new(0), mint::new(0))
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (a.0 + b.0, a.1 + b.1, a.2 + b.2)
        }
    }

    struct G;
    impl MapMonoid for G {
        type F = mint;
        type M = L;
        fn identity_map() -> Self::F {
            mint::new(0)
        }
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            (
                x.0 + mint::new(2) * f * x.1 + x.2 * f * f,
                x.1 + x.2 * f,
                x.2,
            )
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f + g
        }
    }

    let mut sum = LazySegtree::<F>::new(N);
    for i in 0..N {
        sum.set(i, (mint::new(0), mint::new(1)));
    }

    let mut sum_sq = LazySegtree::<G>::new(N);
    for i in 0..N {
        sum_sq.set(i, (mint::new(0), mint::new(0), mint::new(1)));
    }

    for &(l, r, a) in lra.iter() {
        let l = l - 1;
        let r = r - 1;

        sum.apply_range(l..=r, a);
        sum_sq.apply_range(l..=r, a);

        let ans = (sum.prod(l..=r).0.pow(2) - sum_sq.prod(l..=r).0) / mint::new(2);
        println!("{ans}");
    }
}
