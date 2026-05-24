#![allow(non_snake_case)]
use amplify::confinement::Collection;
use proconio::input;
use std::collections::VecDeque;
/*
堀の中にある領域が連結であることが必要
16マスなので、bit全探索でok
堀の外が全て結合しているかを確認する
堀の内側にさらに堀がある状態になっていないことを保証
*/
fn main() {
    input! {
        A: [[usize; 4]; 4],
    }

    let mut ans = 0;
    'outer: for bit in 0..1 << 16 {
        // 全ての村が堀の中であることを確認
        let mut map = vec![vec![0; 6]; 6]; // 0: 堀の外, 1: 堀の中
        let mut cnt1 = 0;
        let mut sh = 0;
        let mut sw = 0;
        for i in 0..16 {
            let h = i / 4;
            let w = i % 4;

            if bit >> i & 1 == 1 {
                map[h + 1][w + 1] = 1;
                sh = h + 1;
                sw = w + 1;
                cnt1 += 1;
            }

            if A[h][w] == 0 {
                continue;
            }

            if A[h][w] == 1 && bit >> i & 1 != 1 {
                continue 'outer;
            }
        }

        // 堀が連結であることを確認する
        let d = [(0, 1), (!0, 0), (0, !0), (1, 0)];

        // 堀の中が連結であることを確認する
        let mut q = VecDeque::new();
        let mut seen = vec![vec![false; 6]; 6];
        seen[sh][sw] = true;
        q.push((sh, sw));
        while let Some((uh, uw)) = q.pop_front() {
            for &(dh, dw) in d.iter() {
                let vh = uh.wrapping_add(dh);
                let vw = uw.wrapping_add(dw);
                if vh >= 6 || vw >= 6 {
                    continue;
                }
                if seen[vh][vw] {
                    continue;
                }
                if map[vh][vw] == 1 {
                    q.push((vh, vw));
                    seen[vh][vw] = true;
                }
            }
        }
        if seen
            .iter()
            .map(|x| x.iter().map(|x| if *x { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>()
            != cnt1
        {
            continue 'outer;
        }

        // 堀が一つであることを確認する
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut seen = vec![vec![false; 6]; 6];
        q.push((0, 0));
        while let Some((uh, uw)) = q.pop_front() {
            for &(dh, dw) in d.iter() {
                let vh = uh.wrapping_add(dh);
                let vw = uw.wrapping_add(dw);
                if vh >= 6 || vw >= 6 {
                    continue;
                }
                if seen[vh][vw] {
                    continue;
                }
                if map[vh][vw] == 0 {
                    q.push((vh, vw));
                    seen[vh][vw] = true;
                }
            }
        }

        let cnt0 = 6 * 6 - cnt1;
        if seen
            .iter()
            .map(|x| x.iter().map(|x| if *x { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>()
            != cnt0
        {
            continue 'outer;
        }
        ans += 1;
    }
    println!("{ans}");
}
