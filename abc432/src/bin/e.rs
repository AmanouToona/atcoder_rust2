#![allow(non_snake_case)]
use ac_library::segtree::Monoid;
use ac_library::Segtree;
use proconio::input;
fn main() {
    input! {
        (N, Q) : (usize, usize),
        mut A: [usize; N],
    }

    struct M;
    impl Monoid for M {
        type S = usize;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
        fn identity() -> Self::S {
            0
        }
    }

    let mut seg_sum = Segtree::<M>::new(5 * 10_0000 + 1);
    let mut seg_cnt = Segtree::<M>::new(5 * 10_0000 + 1);

    for &a in A.iter() {
        seg_sum.set(a, seg_sum.get(a) + a);
        seg_cnt.set(a, seg_cnt.get(a) + 1);
    }

    for _ in 0..Q {
        input! {q : usize}
        match q {
            1 => {
                input! {
                    (x, y): (usize, usize),
                }
                let ax: usize = A[x - 1];

                seg_sum.set(ax, seg_sum.get(ax) - ax);
                seg_sum.set(y, seg_sum.get(y) + y);

                seg_cnt.set(ax, seg_cnt.get(ax) - 1);
                seg_cnt.set(y, seg_cnt.get(y) + 1);

                A[x - 1] = y;
            }
            2 => {
                input! {(l, r): (usize, usize)}
                let ans = if r < l {
                    seg_cnt.prod(..) * l
                } else {
                    seg_cnt.prod(..l) * l + seg_sum.prod(l..r) + seg_cnt.prod(r..) * r
                };

                println!("{ans}");
            }
            _ => {
                panic!()
            }
        }
    }
}
