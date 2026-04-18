#![allow(non_snake_case)]
use ac_library::Dsu;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
/*
ヒープに入れて、報酬が高くなるように連結していく？
罰金を払うことになる辺を取り除くメリットはあるのだろうか？ ... たぶんない。
貪欲ではない気がする。
ある辺を取り除いた後にグラフが連結であることを高速に評価できるだろうか？ ... 無理. 繋げていこう
いや、連結であることを求めるから、貪欲でいいのだ
*/
fn main() {
    input! {
        (N, M): (usize, usize),
        abc: [(usize, usize, i64); M],
    }

    let mut q = BinaryHeap::new();
    for &(a, b, c) in abc.iter() {
        q.push((Reverse(c), a - 1, b - 1));
    }

    let mut dsu = Dsu::new(N);
    let mut ans = 0;
    while let Some((Reverse(c), a, b)) = q.pop() {
        if dsu.same(a, b) {
            ans += 0.max(c);
        } else {
            dsu.merge(a, b);
        }
    }

    println!("{ans}");
}
