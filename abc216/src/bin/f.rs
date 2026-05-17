#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
A, B のどちらかに対してソートしても良い
A に対してソートする
前から i 番目を確認するとき、 Ai が max A となるので、 Ai を max とする条件を満たす集合 S を考えれば良い。
この集合に必要な条件は、
- Ai >= Bi
- Ai >= sig s B - Bi

max という条件を外して簡略化できた。

sig の処理が必要
sig B は 0..=25000_000  <- 2 ** 5000 よりは少ない
5000*3 の計算量なら愚直に解くことはできる

ただ、 Ai よりも小さな値つまり 5000 までだけを考えれば良いので、5000 を超えた値を保持する必要はない。
これで計算量を 5000 ** 2 に落とせる
*/

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }

    let mut ab: Vec<(usize, usize)> = A.iter().zip(B.iter()).map(|(&a, &b)| (a, b)).collect();
    ab.sort_by(|&x, &y| x.0.cmp(&y.0));

    let mut ans = mint::new(0);

    let mut sum_cnt = vec![mint::new(0); 5001];
    sum_cnt[0] = mint::new(1);
    for &(a, b) in ab.iter() {
        if a >= b {
            for i in sum_cnt.iter().take(a - b + 1) {
                ans += i;
            }
        }

        for i in (0..=5000).rev() {
            let nxt = b + i;
            if nxt <= 5000 {
                let tmp = sum_cnt[i];
                sum_cnt[nxt] += tmp;
            }
        }
    }

    println!("{ans}");
}
