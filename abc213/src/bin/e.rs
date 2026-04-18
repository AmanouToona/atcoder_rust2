#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
/*
壊された場所の状態が重なるのが難しい
h, w の最大値が小さい
全部を試す...
ルートを先に決めてしまうのはどうだろうか? ...
1箇所壊した時に4通りの壊し方がある。　それを保存してしまうのは？ qに入れる. 数字を入れて、自分よりも壊した回数が少ない方には移動しない。
コレで行けそう。　証明出来るだろうか
*/

fn main() {
    input! {
        (H, W): (usize, usize),
        S: [Chars; H],
    }

    let mut dp = vec![vec![usize::MAX; W]; H];
    dp[0][0] = 0;
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), 0, 0));
    while let Some((Reverse(cost), uh, uw)) = q.pop() {
        if dp[uh][uw] < cost {
            continue;
        }

        for (dh, dw) in [(0, 1), (0, !0), (1, 0), (!0, 0)].iter() {
            let vh = uh.wrapping_add(*dh);
            let vw = uw.wrapping_add(*dw);

            if vh >= H || vw >= W || dp[vh][vw] < cost {
                continue;
            }

            // 壁を壊す
            if S[vh][vw] == '#' {
                for dh in [!0, 0, 1].iter() {
                    for dw in [!0, 0, 1].iter() {
                        if *dh == 0 && *dw == 0 {
                            continue;
                        }

                        let nxt_h = vh.wrapping_add(*dh);
                        let nxt_w = vw.wrapping_add(*dw);

                        if nxt_h >= H || nxt_w >= W || dp[nxt_h][nxt_w] <= cost + 1 {
                            continue;
                        }
                        q.push((Reverse(cost + 1), nxt_h, nxt_w));
                        dp[nxt_h][nxt_w] = cost + 1;
                    }
                }
            } else if dp[vh][vw] > cost {
                dp[vh][vw] = cost;
                q.push((Reverse(cost), vh, vw));
            }
        }
    }

    println!("{}", dp[H - 1][W - 1]);
}
