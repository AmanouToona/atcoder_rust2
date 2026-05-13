#![allow(non_snake_case)]
use proconio::input;

/*
K が大きいので愚直に2分木を使うことはできない

主客転倒的な考え方？
楽しさが x を下回るまで乗り続ける ...

乗り物の楽しさを x 以下にできるか? を確認すればおおよそ答えになる
ただし、 x 以下にできない場合でも、 x & x + 1 の乗り物が混在するのでこの処理が必要
x + 1 にした後に、残り K 回　乗れば　x & x + 1 の混在状態になる x のみにはならない.
*/

fn gauss_sum(min: usize, max: usize) -> usize {
    (min + max) * (max - min + 1) / 2
}

fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }

    let mut ok = *A.iter().max().unwrap() + 1;
    let mut ng = 0;

    // 本当にng?
    if A.iter().sum::<usize>() <= K {
        let mut ans = 0;
        for &a in A.iter() {
            ans += gauss_sum(0, a);
        }
        println!("{ans}");
        return;
    }

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let mut cost = 0;
        for &a in A.iter() {
            cost += a.saturating_sub(mid);
        }
        if cost <= K {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let mut ans = 0;
    let mut res = K;
    eprintln!("ok : {ok}");
    for &a in A.iter() {
        if a > ok {
            ans += gauss_sum(ok + 1, a);
            res -= a - ok;
        }
    }

    ans += res * ok;
    println!("{ans}");
}
