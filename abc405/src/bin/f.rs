#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;
fn main() {
    input! {
        (n,m ): (usize, usize),
        AB: [(usize, usize); m],
        Q: usize,
        CD: [(usize, usize); Q],
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

    let mut AB: Vec<(usize, usize)> = AB
        .into_iter()
        .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
        .collect();
    AB.sort();

    let CD: Vec<(usize, usize)> = CD
        .into_iter()
        .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
        .collect();
    let mut CD: Vec<(usize, usize, usize)> =
        CD.iter().enumerate().map(|(i, &x)| (x.0, x.1, i)).collect();
    CD.sort_by(|&a, &b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    CD.sort();

    let mut seg = Segtree::<M>::new(2 * n + 1);
    let mut ans = vec![0; Q];
    let mut i = 0;
    for &(c, d, q) in CD.iter() {
        // 更新
        while i < m && AB[i].0 < c {
            seg.set(AB[i].1, 1);
            i += 1;
        }

        ans[q] += seg.prod(c..d);
    }

    let mut seg = Segtree::<M>::new(2 * n + 1);
    AB.sort_by(|a, b| a.1.cmp(&b.1));
    CD.sort_by(|&a, &b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    let mut i = (m - 1) as i64;
    for &(c, d, q) in CD.iter().rev() {
        // 更新
        while i >= 0 && AB[i as usize].1 > d {
            seg.set(AB[i as usize].0, 1);
            i -= 1;
        }

        // 回答
        ans[q] += seg.prod(c..d);
    }

    for a in ans.iter() {
        println!("{a}");
    }
}
