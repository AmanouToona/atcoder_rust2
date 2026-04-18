#![allow(non_snake_case)]
use amplify::confinement::Collection;
use proconio::input;
use std::collections::VecDeque;
fn main() {
    /*
    実は全探索？各辺を使う使わないで... 状態が多いから違う。
    制約を満たす塀は少ない?
    dfs で全探索しようとしたが、領域を作るのが大変。
    bit 全探索に切り替え
    dsu で結合を確認したが、外周部だけ結合などのケースに対応できない
    */
    input! {
        A: [[usize; 4]; 4],
    }

    let mut ans = 0;
    'outer: for bit in 0..(1 << 16) {
        for h in 0..4 {
            for w in 0..4 {
                if A[h][w] == 1 && (bit >> (h * 4 + w)) & 1 == 0 {
                    continue 'outer;
                }
            }
        }

        // bit の 1 の連結性の確認
        let mut count1 = 0;
        let mut sh = 0;
        let mut sw = 0;
        for i in 0..4 {
            for j in 0..4 {
                if bit >> (i * 4 + j) & 1 == 1 {
                    sh = i;
                    sw = j;
                    count1 += 1;
                }
            }
        }
        let count0 = 16 - count1;

        let mut seen = vec![vec![false; 4]; 4];
        let mut q = VecDeque::new();
        let mut seen1 = 0;
        q.push((sh, sw));
        while let Some((uh, uw)) = q.pop_front() {
            if seen[uh][uw] {
                continue;
            }
            if bit >> (uh * 4 + uw) & 1 == 1 {
                seen1 += 1;
            }
            seen[uh][uw] = true;
            for &(dh, dw) in [(0, 1), (!0, 0), (0, !0), (1, 0)].iter() {
                let vh = uh.wrapping_add(dh);
                let vw = uw.wrapping_add(dw);

                if vh >= 4 || vw >= 4 || bit >> (vh * 4 + vw) & 1 == 0 || seen[vh][vw] {
                    continue;
                }
                q.push((vh, vw));
            }
        }

        if seen1 != count1 {
            continue 'outer;
        }

        // 0 の連結性の確認
        let mut seen = vec![vec![false; 6]; 6];
        let mut q = VecDeque::new();
        let mut seen0 = 0;
        q.push((0, 0));
        while let Some((uh, uw)) = q.pop_front() {
            if seen[uh][uw] {
                continue;
            }

            seen[uh][uw] = true;
            if (1..=4).contains(&uh)
                && (1..=4).contains(&uw)
                && bit >> ((uh - 1) * 4 + (uw - 1)) & 1 == 0
            {
                seen0 += 1;
            }

            for &(dh, dw) in [(0, 1), (!0, 0), (0, !0), (1, 0)].iter() {
                let vh = uh.wrapping_add(dh);
                let vw = uw.wrapping_add(dw);

                if vh >= 6 || vw >= 6 || seen[vh][vw] {
                    continue;
                }
                if (1..=4).contains(&vh)
                    && (1..=4).contains(&vw)
                    && bit >> ((vh - 1) * 4 + vw - 1) & 1 == 1
                {
                    continue;
                }

                q.push((vh, vw));
            }
        }
        if seen0 != count0 {
            continue;
        }

        ans += 1;
    }

    println!("{ans}");
}
