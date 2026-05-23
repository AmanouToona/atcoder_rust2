#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;

/*
1はhash set で set.len == N になったら削除して、削除カウンタを ++, setをカラにする
2 各マスの個数の把握は可能だが、高速に y 個以上の個数を求めたい
multiset が使えれば楽だが ...
seg 木で殴ることは可能
*/
fn main() {
    input! {
        (N, Q): (usize, usize),
    }

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

    let mut seg = Segtree::<M>::new(3 * 100_000 + 1);
    seg.set(0, N);

    let mut cnt: Vec<usize> = vec![0; N];

    for _ in 0..Q {
        input! {q: usize }
        match q {
            1 => {
                input! {x : usize}
                let x = x - 1;
                seg.set(cnt[x], seg.get(cnt[x]) - 1);
                cnt[x] += 1;
                seg.set(cnt[x], seg.get(cnt[x]) + 1);
            }
            2 => {
                input! {y: usize}

                if seg.prod(..=0) != 0 {
                    let ans = seg.prod(y..);
                    println!("{ans}");
                } else {
                    let mut ng = 0;
                    let mut ok = 3 * 100_000 + 1;
                    while ok - ng > 1 {
                        let mid = (ng + ok) / 2;
                        if seg.prod(..=mid) != 0 {
                            ok = mid;
                        } else {
                            ng = mid;
                        }
                    }
                    if y + ok > Q {
                        println!("0");
                    } else {
                        let ans = seg.prod(y + ok..);
                        println!("{ans}");
                    }
                }
            }
            _ => {
                panic!()
            }
        }
    }
}
