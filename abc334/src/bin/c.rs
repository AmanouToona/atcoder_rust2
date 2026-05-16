#![allow(non_snake_case)]
use proconio::input;

/*
Ai, Ai+1 を組み合わせる貪欲で良いか？
概ねよいが、一致する色の靴下については一致する色で組み合わせる

使わない靴下はどれか？
左右から貪欲をして、組み合わせる方法で良い

*/
fn main() {
    input! {
        (_, K): (usize, usize),
        A: [usize; K],
    }

    // 左から i 組み作成
    let mut left = vec![0];
    for (&i, &j) in A.iter().step_by(2).zip(A.iter().skip(1).step_by(2)) {
        left.push(left.last().unwrap() + j - i);
    }

    // 左から i　組み作成
    let mut right = vec![0];
    for (&i, &j) in A
        .iter()
        .rev()
        .step_by(2)
        .zip(A.iter().rev().skip(1).step_by(2))
    {
        right.push(right.last().unwrap() + i - j);
    }

    // 合計で片割れ靴下では K / 2 組み作成するので
    let mut ans = usize::MAX;
    for (i, &l) in left.iter().enumerate() {
        ans = ans.min(l + right[K / 2 - i]);
    }

    println!("{ans}");
}
