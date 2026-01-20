use proconio::input;
use std::collections::BTreeMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K, X): (usize, i64, usize),
        mut A: [i64; N],
    }
    A.sort_by(|x, y| y.cmp(&x));
    let mut q = BTreeMap::new();

    if N >= 2 {
        q.insert(A[0] - A[1], (K, 0, 1));
    }
    let mut ans = A[0] * K;

    for _ in 0..X {
        println!("{ans}");

        for (&k, v) in q.iter_mut() {
            if v.0 == 0 {
                continue;
            }

            ans -= v.0;
            v.0 -= 1;

            if v.2 + 1 < N {}
        }
    }
}
