#![allow(non_snake_case)]
use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

/*
一定の状態に落ちる？
振動する例がある. 一定状態にはならない

10 ** 100 なので、愚直は無理 ダブリング？
状態を key に持つ？ ... 状態数が多い 2 ** (10**6) なので現実的ではない

各マスに着目すると、黒になった後は、周期的な動きをする ... 各マスが少なくとも1回黒になるまで実験すれば良い
10 ** 100 は 2で割れるのでパリティが利用可能

sample2が特殊な例
黒にのみ囲まれた領域の挙動を考える必要がある
遷移途中で、自身が黒にのみ囲まれることがあり得るか？ -> 一つ前の状態で少なくとの1箇所は白なので、それは発生しない

*/
fn main() {
    input! {
        (H, W): (usize, usize),
        S: [Chars; H],
    }

    let d: [(usize, usize); 8] = [
        (0, 1),
        (!0, 1),
        (!0, 0),
        (!0, !0),
        (0, !0),
        (1, !0),
        (1, 0),
        (1, 1),
    ];

    let mut blacked = 0;
    let mut black_parity: Vec<Vec<Option<i32>>> = vec![vec![None; W]; H];
    let mut q = VecDeque::new();
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '#' {
                // 1回遷移させる
                for (dh, dw) in d.iter() {
                    let uh = h.wrapping_add(*dh);
                    let uw = w.wrapping_add(*dw);
                    if uh >= H || uw >= W {
                        continue;
                    }
                    if S[uh][uw] == '#' {
                        continue;
                    }
                    q.push((uh, uw));
                    blacked += 1;
                    black_parity[uh][uw] = Some(1);
                }
            }
        }
    }

    while blacked != H * W && !q.is_empty() {
        let mut nxt_q: VecDeque<(usize, usize)> = VecDeque::new();
        while let Some((uh, uw)) = q.pop_front() {
            for (dh, dw) in [
                (0, 1),
                (!0, 1),
                (!0, 0),
                (!0, !0),
                (0, !0),
                (1, !0),
                (1, 0),
                (1, 1),
            ] {
                let vh = uh.wrapping_add(dh);
                let vw = uw.wrapping_add(dw);

                if vh >= H || vw >= W {
                    continue;
                }

                if black_parity[vh][vw].is_some() {
                    continue;
                }

                let p = black_parity[uh][uw].unwrap();
                black_parity[vh][vw] = Some(1 - p);
                blacked += 1;
                nxt_q.push((vh, vw));
            }
        }
        q = nxt_q;
    }

    let mut ans = vec![Vec::new(); H];
    for h in 0..H {
        for w in 0..W {
            if black_parity[h][w] == Some(0) {
                ans[h].push('#');
            } else {
                ans[h].push('.');
            }
        }
    }

    for i in ans.iter() {
        let a: String = i.iter().join("");
        println!("{a}");
    }
}
