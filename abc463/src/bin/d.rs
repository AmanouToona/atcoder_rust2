#![allow(non_snake_case)]
use proconio::input;

/*
K まいの選び方も組み合わせが多いので、選ぶことはできない

状態を持ちたいが、状態はなんだろうか？
布長さ、　布の左端、 布の右端、 どこまで見たか、　何枚選択したか
2分探索? ...
*/

fn can_select(gap: usize, k: usize, lr: &[(usize, usize)]) -> bool {
    let mut right = lr[0].1;
    let mut cnt = 1;

    for &(l, r) in lr.iter().skip(1) {
        if l >= right + gap {
            right = r;
            cnt += 1;
        }
    }

    cnt >= k
}

fn main() {
    input! {
        (N, K): (usize, usize),
        mut lr: [(usize, usize); N],
    }

    lr.sort_by_key(|x| x.0);
    lr.sort_by_key(|x| x.1);

    if !can_select(1, K, &lr) {
        println!("-1");
        return;
    }

    let mut ok = 1;
    let mut ng = 1_000_000_000;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if can_select(mid, K, &lr) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}
