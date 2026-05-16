#![allow(non_snake_case)]
use proconio::input;

/*
最小値の最大化 ... 典型 2分探索

10**18 ... 2 ** 60 くらい
N = 2 * 10 ** 5

計算量は N * 2 ** 60  なので余裕
*/

fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }

    let mut ok = *A.iter().min().unwrap();
    let mut ng = 10usize.pow(24);

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        let mut cost = 0;
        for (i, &a) in A.iter().enumerate() {
            cost += mid.saturating_sub(a).div_ceil(i + 1);
        }

        if cost <= K {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{ok}");
}
