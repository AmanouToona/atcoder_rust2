#![allow(non_snake_case)]
use std::iter;

use itertools::Itertools;
use num::Integer;
use proconio::input;
/*
市松模様を考える。
- 初期位置と最終位置の色は同じ 白色とする。
- 移動の度に色が変わる。初期位置と最終位置が白なので、黒色は、白の個数 - 1 出ないと塗りきれない。
- 全体で通るマスの個数は奇数であることが必要とわかる. N ** 2 - 1 が奇数 -> N は偶数
- 禁止箇所は黒色である必要がある。　だから、 h + w が奇数のマスを禁止箇所とする
- 小さな問題に帰着できる

*/
fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            (N, A, B): (usize, usize, usize),
        }

        let mut A = A - 1;
        let mut B = B - 1;

        if N.is_odd() || (A + B).is_even() {
            println!("No");
            continue;
        }

        let mut s1: Vec<char> = Vec::new();
        let mut s2: Vec<char> = Vec::new();
        let mut H = N - 1;
        let mut W = N - 1;

        // 行を減らす
        while A >= 2 {
            s1.extend(iter::repeat_n('R', W));
            s1.push('D');
            s1.extend(iter::repeat_n('L', W));
            s1.push('D');

            H -= 2;
            A -= 2;
        }

        while H > 1 {
            s2.extend(iter::repeat_n('R', W));
            s2.push('D');
            s2.extend(iter::repeat_n('L', W));
            s2.push('D');
            H -= 2;
        }

        // 列を減らす
        while B >= 2 {
            s1.extend(iter::repeat_n('D', H));
            s1.push('R');
            s1.extend(iter::repeat_n('U', H));
            s1.push('R');

            W -= 2;
            B -= 2;
        }

        while W > 1 {
            s2.push('D');
            s2.push('R');
            s2.push('U');
            s2.push('R');
            W -= 2;
        }

        // 最終的な小ささ問題の解を加える
        match (A, B) {
            (1, 0) => {
                s1.extend(['R', 'D'].iter());
            }
            (0, 1) => {
                s1.extend(['D', 'R'].iter());
            }
            _ => {
                panic!()
            }
        }

        let ans: String = s1.iter().chain(s2.iter().rev()).join("");

        println!("Yes");
        println!("{ans}");
    }
}
