#![allow(non_snake_case)]
use proconio::input;

/*
A: 1, K
B: K only

A, B を sort
安いものから購入する

... B を購入する個数を決め打つ？
購入に必要な K の枚数とあまりの 1ドル紙幣の枚数は高速に算出可能
2分探索ができるので、計算量は O(M logN) ... 間に合う
*/

fn main() {
    input! {
        (N, M, K): (usize, usize, usize),
        (X, Y): (usize, usize),
        mut A: [usize; N],
        mut B: [usize; M],
    }

    A.sort();
    let mut cumsum_a = vec![0];
    cumsum_a.extend(A);
    for i in 0..N {
        cumsum_a[i + 1] += cumsum_a[i];
    }

    B.sort();

    let mut ans = 0;
    let mut k_use = 0;
    let mut tot_b = 0;

    for (i, &b) in std::iter::once(&0).chain(B.iter()).enumerate() {
        k_use += b.div_ceil(K);
        tot_b += b;

        if k_use > Y {
            break;
        }

        let res = X + (Y * K - tot_b);

        ans = ans.max(i + cumsum_a.partition_point(|x| *x <= res) - 1);
    }

    println!("{ans}");
}
