#![allow(non_snake_case)]
use proconio::input;

/*
状態数
トナカイの状態の数は 2 ** N
力の状態の数は 3 * 10 ** 14
ソリにいるトナカイの状態は N頭　ソリで、余力の最大値

全てソリに乗った状態から開始する
1頭外に出すと w + p だけ軽くなる
0 を超えるまで外に出す
*/

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            N: usize,
            wp: [(usize, usize); N],
        }

        let mut ratio: Vec<usize> = wp.iter().map(|x| x.0 + x.1).collect();
        ratio.sort_by(|x, y| y.cmp(x));

        let mut sled: i128 = wp.iter().map(|&x| x.0).sum::<usize>() as i128;

        let mut out = 0;
        while sled > 0 {
            sled -= ratio[out] as i128;
            out += 1;
        }

        println!("{}", N - out);
    }
}
