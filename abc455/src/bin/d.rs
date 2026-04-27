#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
/*
pile, card, position が要素

山の頂点のカードがどこにあるか？
山には幾つのカードがあるか？
カードの上には幾つのカードがあるか？
カードの下のカードは何か？

カードの下のカードだけ把握しておけば復元できる
山は、マイナス番号とすると楽か？

*/

fn main() {
    input! {
        (N, Q): (usize, usize),
        cp: [(usize, i64 ); Q],
    }

    let mut floor = vec![0; N + 1];
    for i in 0..=N {
        floor[i] = -(i as i64);
    }

    for &(c, p) in cp.iter() {
        floor[c] = p;
    }
    // eprintln!("{:?}", floor);

    let mut ans = vec![1; N + 1];
    let mut used = vec![false; N + 1];
    for i in 1..=N {
        let mut c = floor[i];
        if c < 0 {
            continue;
        }
        ans[i] = 0;
        if used[i] {
            continue;
        }

        used[i] = true;
        let mut cnt = 1;
        while c > 0 {
            used[c.abs() as usize] = true;
            c = floor[c as usize];
            cnt += 1;
        }
        ans[c.unsigned_abs() as usize] = cnt;
    }

    let ans: String = ans.iter().skip(1).join(" ");
    println!("{ans}");
}
