use ac_library::segtree::Monoid;
use ac_library::Segtree;
use proconio::input;

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

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        B: [usize; N]
    }

    let mut B: Vec<(usize, usize)> = B.iter().cloned().enumerate().collect();
    B.sort_by(|x, y| x.1.cmp(&y.1));

    let mut seg = Segtree::<M>::new(N + 1);
    let mut ans = 0;
    for &(i, _) in B.iter() {
        seg.set(i, 1);

        let left = seg.prod(0..i) + 1;
        let right = seg.prod(i + 1..=N) + 1;
        ans += left * right;
    }

    println!("{ans}");
}
