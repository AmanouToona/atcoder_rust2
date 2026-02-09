#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        A: [usize; N],
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

    let mut seg = Segtree::<M>::new(N + 1);
    for (i, &a) in A.iter().enumerate() {
        seg.set(i + 1, a);
    }

    for _ in 0..Q {
        input! {
            q: usize
        }
        match q {
            1 => {
                input! {x: usize}

                let l = seg.get(x);
                let r = seg.get(x + 1);
                seg.set(x, r);
                seg.set(x + 1, l);
            }
            2 => {
                input! {(l, r): (usize, usize)}
                let ans = seg.prod(l..=r);
                println!("{ans}");
            }
            _ => {}
        }
    }
}
