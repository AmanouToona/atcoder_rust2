#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;

/*
デコ x と　ボコ y をカウントし x > y か判定する
デコで始まり、デコで終わる ... これで条件を満たす (1個のみでも可)

デコになり得る部分を抽出する？

pi と pj がそれぞれは時の山となる状態をカウントする
- pi を山するには pi より小さく出現済みのものをカウント
- pj を和するには、pj より小さく出現前のものをカウント
- j をインクリメントする
- j += 1 ...  sum(piを山とするpiより前に出現したものの組み合わせ) * 2 * (pj を山とする組み合わせ)
  - j も i に含めると楽そう sum(pi) * 2 + (pjより小さな出現済みのものの選択方法)

場合分け
i == j の時の処理を同列に扱えないので場合分けする
- pi = a2, pj = a(k - 1)
- pi = pj = a2
*/

fn main() {
    input! {
        N: usize,
        P: [usize; N],
    };

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

    let mut left = Segtree::<M>::new(N + 1);
    let mut right = Segtree::<M>::new(N + 1);
    for i in 1..=N {
        right.set(i, 1);
    }

    let mut ans = mint::new(0);
    let mut sum_ai = mint::new(0);

    for &p in P.iter() {
        left.set(p, 1);
        right.set(p, 0);

        let l = left.prod(..p);
        let r = right.prod(..p);

        // i == j
        ans += mint::new(l * r);

        // i < j
        sum_ai *= mint::new(2);
        ans += sum_ai * r;
        sum_ai += mint::new(l) / mint::new(2);
    }
    println!("{ans}");
}
