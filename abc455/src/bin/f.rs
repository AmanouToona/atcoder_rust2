#![allow(non_snake_case)]
use ac_library::LazySegtree;
use ac_library::ModInt998244353 as mint;
use ac_library::{MapMonoid, Monoid};
use proconio::input;
/*
最終コストについて、 a, b, c, d のスライムを頭からくっ付けていくと
a(b + c + d) + b(c + d) + c(d) のようになる。
これを最小化したい ... 順番によって変化する？　変化しないのでは？

変化しないとして、これを高速に計算する方法はあるだろうか？
上式を2倍して整理すると
a(b + c + d) + b(a + c + d) + c(a + b+ d) + d(a + b + c)
という綺麗な形になる さらに、 整理すると
(a + b + c + d) ** 2 - (a ** 2 + b ** 2 + c ** 2 + d ** 2)
という綺麗な形になる

2乗の和の方が難しいが、単純な和算の2乗の方については遅延セグ木で処理できる

2乗の方を遅延セグ木に載せる方法は？
*/

fn main() {
    input! {
        (N, Q): (usize, usize),
        lra: [(usize, usize, usize); Q],
    }

    struct M;
    impl Monoid for M {
        type S = (mint, mint);
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (a.0 + b.0, a.1 + b.1)
        }
        fn identity() -> Self::S {
            (mint::new(0), mint::new(0))
        }
    }

    struct F;
    impl MapMonoid for F {
        type M = M;
        type F = mint;

        fn identity_map() -> Self::F {
            mint::new(0)
        }

        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            (x.0 + *f * x.1, x.1)
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            *f + *g
        }
    }

    struct L;
    impl Monoid for L {
        type S = (mint, mint, mint); //ans, a**2, a, size
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (a.0 + b.0, a.1 + b.1, a.2 + b.2)
        }
        fn identity() -> Self::S {
            (mint::new(0), mint::new(0), mint::new(0))
        }
    }

    struct G;
    impl MapMonoid for G {
        type M = L;
        type F = mint;
        fn identity_map() -> Self::F {
            mint::new(0)
        }
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            (
                x.0 + mint::new(2) * f * x.1 + x.2 * f * f,
                x.1 + f * x.2,
                x.2,
            )
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f + g
        }
    }

    let mut seg_sum = LazySegtree::<F>::new(N);
    let mut seg_pow = LazySegtree::<G>::new(N);

    for i in 0..N {
        seg_sum.set(i, (mint::new(0), mint::new(1)));
        seg_pow.set(i, (mint::new(0), mint::new(0), mint::new(1)));
    }

    for &(l, r, a) in lra.iter() {
        let l = l - 1;
        let r = r - 1;

        seg_sum.apply_range(l..=r, mint::new(a));
        seg_pow.apply_range(l..=r, mint::new(a));

        let s = seg_sum.prod(l..=r);
        let p = seg_pow.prod(l..=r);

        let ans =
            (seg_sum.prod(l..=r).0 * seg_sum.prod(l..=r).0 - seg_pow.prod(l..=r).0) / mint::new(2);
        println!("{ans}");
    }
}
