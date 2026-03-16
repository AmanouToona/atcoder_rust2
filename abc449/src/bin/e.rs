#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N],
        Q: usize,
        X: [usize; Q],
    }

    let mut cnt: Vec<usize> = vec![0; M + 1];
    for &a in A.iter() {
        cnt[a] += 1;
    }

    let mut cnt: Vec<(usize, usize)> = cnt
        .into_iter()
        .enumerate()
        .skip(1)
        .map(|x| (x.1, x.0))
        .collect();
    cnt.sort();

    let mut cnt2num: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &(cnt, num) in cnt.iter() {
        cnt2num.entry(cnt).or_default().push(num); // なぜ derefarence いらないのだろ？
    }
    cnt2num.entry(usize::MAX).or_default(); // sentinel

    let mut X: Vec<(usize, usize)> = X.iter().enumerate().map(|x| (*x.1, x.0)).collect();
    X.sort();
    let mut ans = vec![0; Q];
    let mut i = 0;
    // 末尾の加工が不要な部分は先に答えを得る
    while i < Q {
        if X[i].0 <= N {
            ans[X[i].1] = A[X[i].0 - 1];
            i += 1;
        } else {
            break;
        }
    }

    let mut len = N;

    struct S;
    impl Monoid for S {
        type S = usize;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
        fn identity() -> Self::S {
            0
        }
    }

    let mut tail = Segtree::<S>::new(M + 1);
    let mut tail_len = 0;

    for ((&cnt, num), (&cnt2, _)) in cnt2num.iter().zip(cnt2num.iter().skip(1)) {
        tail_len += num.len();
        for &n in num {
            tail.set(n, 1);
        }
        while i < Q {
            let (x, idx) = X[i];
            if (x - len).div_ceil(tail_len) <= cnt2 - cnt {
                let j = (x - 1 - len) % tail_len;
                // tail ないの j 番目の要素
                ans[idx] = tail.max_right(0, |&sum| sum <= j);

                i += 1;
            } else {
                break;
            }
        }
        if cnt2 == usize::MAX || i == Q {
            break;
        }
        len += tail_len * (cnt2 - cnt);
    }

    for i in ans {
        println!("{i}");
    }
}
