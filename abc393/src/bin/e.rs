#![allow(non_snake_case)]
use proconio::input;
/*
素因数で考える
osa で素因数の列挙は十分高速に可能

nCk は組み合わせが多いので愚直は不可能
また、すべての Ai についての解が必要

逆に g が条件を満たすかを判定する
これは計算量 max(A)logN でできるので時間内で実行可能

ai の約数で条件を満たす最大のものを探索すれば良い
これはもう一度 max(A)logN の動きをしつつ　[i] = max(i - n) で更新していけば良い

*/

fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }

    let a_max = *A.iter().max().unwrap();

    let mut a_cnt = vec![0; a_max + 1];
    for &a in A.iter() {
        a_cnt[a] += 1;
    }

    let mut g = vec![false; a_max + 1];
    for i in 1..=a_max {
        let mut j = i;
        let mut cnt = 0;
        while j <= a_max {
            cnt += a_cnt[j];
            j += i;
        }

        if cnt >= K {
            g[i] = true;
        }
    }

    let mut ans = vec![1; a_max + 1];
    for i in 1..=a_max {
        if g[i] {
            let mut j = i;
            while j <= a_max {
                ans[j] = i;
                j += i;
            }
        }
    }

    for &a in A.iter() {
        println!("{}", ans[a]);
    }
}
