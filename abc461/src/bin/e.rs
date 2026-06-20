#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;

/*
過去のクエリの影響が消えるところがある？
クエリの順番に依存する。 白をさらに白くしても意味がない
r1 -> c1 -> r1
r1 -> r1 -> c1 は異なる結果になる

ある列に対して行った白色か処理について、その白色化処理の一回前の白色化処理よりも新しい黒色化処理のみ影響 ... セグ木?
セグ木 1回で計算量は log(3 * 10 ** 5) まにあうな

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

    let mut row_q: Vec<Vec<usize>> = vec![vec![0]; N];
    let mut col_q: Vec<Vec<usize>> = vec![vec![0]; N];
    let mut row_seg: Segtree<M> = Segtree::<M>::new(Q + 2);
    let mut col_seg: Segtree<M> = Segtree::<M>::new(Q + 2);
    col_seg.set(0, N);
    let mut black: usize = 0;

    for i in 1..=Q {
        input! {q: usize}
        match q {
            1 => {
                input! {r: usize}
                let r = r - 1;
                let last = row_q[r].pop().unwrap();
                row_q[r].push(i);
                black += col_seg.prod(last..);
                row_seg.set(i, 1);
                row_seg.set(last, 0);
            }
            2 => {
                input! {c: usize}
                let c = c - 1;
                let last = col_q[c].pop().unwrap();
                col_q[c].push(i);
                black -= row_seg.prod(last..);
                col_seg.set(i, 1);
                col_seg.set(last, col_seg.get(last) - 1);
            }
            _ => {
                panic!()
            }
        }
        println!("{black}");
    }
}
